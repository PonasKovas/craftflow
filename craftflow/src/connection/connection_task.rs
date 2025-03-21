mod reader;
mod writer;

use super::{
	ConnectionInterface, State,
	legacy::{LegacyPing, detect_legacy_ping, write_legacy_response},
	packet_reader::PacketReader,
	packet_writer::PacketWriter,
};
use crate::{CraftFlow, various_events::UnsupportedClientVersion};
use anyhow::{Context, bail};
use craftflow_protocol::SUPPORTED_VERSIONS;
use reader::reader_task;
use shallowclone::MakeOwned;
use std::{
	net::SocketAddr,
	ops::ControlFlow,
	sync::{Arc, OnceLock, RwLock},
	time::Duration,
};
use tokio::{net::TcpStream, select, spawn, sync::mpsc, time::timeout};
use tracing::error;
use writer::writer_task;

const CONCRETE_PACKET_CHANNEL_SIZE: usize = 16;
const ABSTRACT_PACKET_CHANNEL_SIZE: usize = 16;

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

	// normally we dont make packets owned, but here we have to because of how the event triggers are spaced out
	let handshake_ab = AbHandshake::construct(&handshake)?
		.assume_done()
		.make_owned();

	let next_state = match handshake_ab.next_state {
		NextState::Status => State::Status,
		NextState::Login | NextState::Transfer => State::Login,
	};

	// unless the next state is status, we need to check that the client protocol version is supported
	if handshake_ab.next_state != NextState::Status {
		if !(MIN_VERSION..=MAX_VERSION).contains(&handshake_ab.protocol_version) {
			let message = match craftflow
				.reactor
				.trigger::<UnsupportedClientVersion>(
					&craftflow,
					&mut (socket_addr.ip(), handshake_ab.protocol_version),
				)
				.await
			{
				ControlFlow::Continue(_) => {
					// default response
					text!("Your version is not supported.", color = "white", bold)
				}
				ControlFlow::Break(message) => message,
			};

			let abs_pkt = AbDisconnect { message };
			let concrete_pkt = abs_pkt
				.convert(handshake_ab.protocol_version, State::Login)?
				.assume_success()
				.next()
				.unwrap();
			packet_writer
				.send(
					next_state,
					handshake_ab.protocol_version,
					None,
					&mut None,
					&concrete_pkt,
				)
				.await?;

			return Ok(()); // close the connection
		}
	}

	// All is good, can add to the client list now and spawn the tasks for reading/writing to it
	let (concrete_packet_sender_in, concrete_packet_sender_out) =
		mpsc::channel(CONCRETE_PACKET_CHANNEL_SIZE);
	let (abstract_packet_sender_in, abstract_packet_sender_out) =
		mpsc::channel(ABSTRACT_PACKET_CHANNEL_SIZE);
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
				protocol_version: handshake_ab.protocol_version,
				concrete_packet_sender: concrete_packet_sender_in,
				abstract_packet_sender: abstract_packet_sender_in,
				encryption_secret: Arc::clone(&encryption_secret),
				compression: Arc::clone(&compression),
				writer_state: Arc::clone(&writer_state),
			}),
		);

		id
	};

	let conn_info = ConnectionInfo {
		id,
		version: handshake_ab.protocol_version,
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
			concrete_packet_sender_out,
			abstract_packet_sender_out,
			conn_info,
		)
		.await
	});

	// now that the tasks are up and running and everything is ready
	// just emit the handshake events for consistency with all other packets

	let handshake_ab: AbC2S = handshake_ab.into();
	'events: {
		let (cont, handshake) = trigger_c2s_concrete(false, &craftflow, id, handshake).await;
		if !cont {
			break 'events;
		}
		let (cont, handshake_ab) = trigger_c2s_abstract(false, &craftflow, id, handshake_ab).await;
		if !cont {
			break 'events;
		}
		let (cont, _handshake_ab) = trigger_c2s_abstract(true, &craftflow, id, handshake_ab).await;
		if !cont {
			break 'events;
		}
		let (cont, _handshake) = trigger_c2s_concrete(true, &craftflow, id, handshake).await;
		if !cont {
			break 'events;
		}
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
			error!("connection task error: {e:?}");
		}
		Err(e) => {
			// panicked... wow.. cringe
			error!("connection task panicked: {e:?}");
		}
	}

	// remove the connection from the list
	craftflow.disconnect(id).await;

	Ok(())
}
