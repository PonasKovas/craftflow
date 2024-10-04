mod reader;
mod writer;

use super::{
	legacy::{detect_legacy_ping, write_legacy_response, LegacyPing},
	packet_reader::PacketReader,
	packet_writer::PacketWriter,
	ConnState,
};
use crate::{
	packet_events::trigger_c2s,
	packets::{C2SPacket, S2CPacket},
	CraftFlow,
};
use anyhow::{bail, Context};
use craftflow_protocol_abstract::{
	c2s::{handshake::NextState, AbHandshake},
	AbPacketNew,
};
use craftflow_protocol_versions::{MAX_VERSION, MIN_VERSION};
use reader::reader_task;
use std::{
	ops::ControlFlow,
	sync::{Arc, OnceLock, RwLock},
	time::Duration,
};
use tokio::{spawn, sync::mpsc::UnboundedReceiver, time::timeout};
use tracing::error;
use writer::writer_task;

/// The task that handles the connection and later splits into two tasks: reader and writer.
pub(super) async fn connection_task(
	craftflow: Arc<CraftFlow>,
	conn_id: u64,
	mut reader: PacketReader,
	mut writer: PacketWriter,
	packet_sender: UnboundedReceiver<S2CPacket>,
	protocol_version: Arc<OnceLock<u32>>,
	state: Arc<RwLock<ConnState>>,
) -> anyhow::Result<()> {
	// First things first check if this is a legacy ping
	if let Some(legacy_ping_format) = detect_legacy_ping(&mut reader.stream).await? {
		// Trigger the legacy ping event
		if let ControlFlow::Break(response) =
			craftflow.reactor.event::<LegacyPing>(&craftflow, conn_id)
		{
			if let Some(response) = response {
				write_legacy_response(&mut writer.stream, legacy_ping_format, response).await?;
			}

			return Ok(()); // close the connection
		}
	}

	// Ok so its not a legacy ping, lets continue with the normal handshake

	// we will read the handshake in this task before splitting into two tasks
	// so we know the next state for both tasks

	let handshake = match timeout(Duration::from_secs(5), reader.read_packet()).await {
		Ok(p) => p.context("reading handshake packet")?,
		Err(_) => bail!("timed out"),
	};

	let handshake_ab = AbHandshake::construct(handshake.clone())
		.unwrap()
		.assume_done();

	// set the client protocol version
	let client_version = handshake_ab.protocol_version;
	protocol_version
		.set(client_version)
		.expect("client protocol version already set");

	let next_state = match handshake_ab.next_state {
		NextState::Status => ConnState::Status,
		NextState::Login | NextState::Transfer => {
			// for these states, check if the client protocol version is actually supported
			if !(MIN_VERSION <= client_version && client_version >= MAX_VERSION) {
				bail!("unsupported protocol version");
			}

			ConnState::Login
		}
	};

	// update the state of the reader and writer
	*state.write().unwrap() = next_state;

	// trigger the handshake event
	if trigger_c2s(&craftflow, conn_id, &mut C2SPacket::Concrete(handshake)).is_continue() {
		trigger_c2s(
			&craftflow,
			conn_id,
			&mut C2SPacket::Abstract(handshake_ab.into()),
		);
	}

	// now we can finally split into two tasks
	// spawn a task to handle writing packets
	// since we now know the state
	let cf_clone = Arc::clone(&craftflow);
	spawn(async move {
		if let Err(e) = writer_task(cf_clone, conn_id, writer, packet_sender).await {
			error!("writer task: {e}");
		}
	});

	// continue reading packets in this task
	if let Err(e) = reader_task(craftflow, conn_id, reader).await {
		error!("reader task: {e}");
	}

	Ok(())
}
