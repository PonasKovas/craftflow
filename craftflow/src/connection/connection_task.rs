mod reader;
mod writer;

use super::{
	ConnectionInterface, State,
	legacy::{LegacyPing, detect_legacy_ping, write_legacy_response},
	packet_reader::PacketReader,
	packet_writer::PacketWriter,
};
use crate::{CraftFlow, packet_events::trigger_c2s, various_events::UnsupportedClientVersion};
use anyhow::{Context, bail};
use craftflow_protocol::{
	C2S, SUPPORTED_VERSIONS,
	c2s::{Handshaking, handshaking::SetProtocol},
	disabled_versions,
	s2c::login::{self, disconnect::v5::DisconnectV5},
};
use reader::reader_task;
use std::{
	net::SocketAddr,
	ops::ControlFlow,
	sync::{Arc, OnceLock, RwLock},
	time::Duration,
};
use tokio::{net::TcpStream, select, spawn, sync::mpsc, time::timeout};
use tracing::error;
use writer::writer_task;

const PACKET_CHANNEL_SIZE: usize = 16;

#[derive(Clone, Debug)]
struct ConnectionInfo {
	id: u64,
	version: u32,
	compression: Arc<OnceLock<usize>>,
	encryption_secret: Arc<OnceLock<[u8; 16]>>,
	reader_state: Arc<RwLock<State>>,
	writer_state: Arc<RwLock<State>>,
}

/// Handles a fresh connection, managing handshake and adding to the client list
pub(crate) async fn handle_new_conn(
	craftflow: Arc<CraftFlow>,
	mut stream: TcpStream,
	socket_addr: SocketAddr,
) -> anyhow::Result<()> {
	// First things first check if this is a legacy ping
	if let Some(legacy_ping_format) = detect_legacy_ping(&mut stream).await? {
		// Trigger the legacy ping event
		if let ControlFlow::Break(response) = craftflow
			.reactor
			.trigger::<LegacyPing>(&craftflow, &mut socket_addr.ip())
			.await
		{
			if let Some(response) = response {
				write_legacy_response(&mut stream, legacy_ping_format, response).await?;
			}

			return Ok(()); // close the connection
		}
	}

	// Ok so its NOT a legacy ping, lets continue with the normal handshake

	// we will read the handshake in this task before splitting into two tasks
	// so we know the next state for both tasks

	let (reader, writer) = stream.into_split();

	let mut packet_reader = PacketReader::new(reader);
	let mut packet_writer = PacketWriter::new(writer);

	let handshake = match timeout(
		Duration::from_secs(5),
		packet_reader.read_packet(State::Handshake, SUPPORTED_VERSIONS[0], None, &mut None),
	)
	.await
	{
		Ok(r) => match r.context("reading handshake packet")? {
			Some(p) => p,
			None => {
				bail!("connection closed before handshake was received");
			}
		},
		Err(_) => bail!("timed out trying to read handshake"),
	};

	let C2S::Handshaking(Handshaking::SetProtocol(SetProtocol::V5(set_protocol))) = handshake
	else {
		unreachable!("there is only one packet in the handshaking state");
	};

	let next_state = match set_protocol.next_state {
		1 => State::Status,
		2 | 3 => State::Login, // 2 - login, 3 - transfer
		_ => bail!("invalid next_state"),
	};
	let version = set_protocol.protocol_version as u32;

	// unless the next state is status, we need to check that the client protocol version is supported
	if next_state != State::Status {
		if !SUPPORTED_VERSIONS.contains(&version) {
			let message = match craftflow
				.reactor
				.trigger::<UnsupportedClientVersion>(&craftflow, &mut (socket_addr.ip(), version))
				.await
			{
				ControlFlow::Continue(_) => {
					// default response
					"Your version is not supported.".to_string()
				}
				ControlFlow::Break(message) => message,
			};

			let disconnect = match login::DisconnectBuilder::new(version) {
				login::DisconnectBuilder::V5(p) => p(DisconnectV5 { reason: message }),
				disabled_versions!(s2c::login::DisconnectBuilder) => unreachable!(),
			}
			.into();

			packet_writer
				.send(next_state, version, None, &mut None, &disconnect)
				.await?;

			return Ok(()); // close the connection
		}
	}

	// All is good, can add to the client list now and spawn the tasks for reading/writing to it
	let (packet_sender_in, packet_sender_out) = mpsc::channel(PACKET_CHANNEL_SIZE);
	let reader_state = Arc::new(RwLock::new(next_state));
	let writer_state = Arc::new(RwLock::new(next_state));
	let compression = Arc::new(OnceLock::new());
	let encryption_secret = Arc::new(OnceLock::new());
	let id = {
		let mut lock = craftflow.connections.write().unwrap();

		let id = lock.next_conn_id;
		lock.next_conn_id += 1;

		lock.connections.insert(
			id,
			Arc::new(ConnectionInterface {
				id,
				ip: socket_addr.ip(),
				protocol_version: version,
				packet_sender: packet_sender_in,
				encryption_secret: Arc::clone(&encryption_secret),
				compression: Arc::clone(&compression),
				writer_state: Arc::clone(&writer_state),
			}),
		);

		id
	};

	let conn_info = ConnectionInfo {
		id,
		version,
		compression,
		encryption_secret,
		reader_state,
		writer_state,
	};
	let conn_info_clone = conn_info.clone();

	let craftflow_clone = Arc::clone(&craftflow);
	let craftflow_clone2 = Arc::clone(&craftflow);
	let reader_task =
		spawn(async move { reader_task(craftflow_clone, packet_reader, conn_info_clone).await });
	let writer_task = spawn(async move {
		writer_task(
			craftflow_clone2,
			packet_writer,
			packet_sender_out,
			conn_info,
		)
		.await
	});

	// now that the tasks are up and running and everything is ready
	// just emit the handshake events for consistency with all other packets

	let handshake = set_protocol.into();
	let (cont, handshake) = trigger_c2s(false, &craftflow, id, handshake).await;
	if cont {
		trigger_c2s(true, &craftflow, id, handshake).await;
	}

	// and now just wait for the tasks to finish for any reason and clean up
	let reader_abort = reader_task.abort_handle();
	let writer_abort = writer_task.abort_handle();

	let result = select! {
		r = reader_task => r.map(|inner| inner.context("reader task")).context("reader task"),
		r = writer_task => r.map(|inner| inner.context("writer task")).context("writer task"),
	};

	// generally i dont condone abortions but in this case its fine
	reader_abort.abort();
	writer_abort.abort();

	match result {
		Ok(Ok(_)) => {} // ended peacefully 😊
		Ok(Err(e)) => {
			error!("connection task error: {e}");
		}
		Err(e) => {
			// panicked... wow.. cringe
			error!("connection task panicked: {e}");
		}
	}

	// remove the connection from the list
	craftflow.disconnect(id).await;

	Ok(())
}
