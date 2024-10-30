mod reader;
mod writer;

use super::{
	legacy::{detect_legacy_ping, write_legacy_response, LegacyPing},
	packet_reader::PacketReader,
	packet_writer::PacketWriter,
};
use crate::{
	packet_events::{trigger_c2s_abstract, trigger_c2s_concrete},
	various_events::UnsupportedClientVersion,
	CraftFlow,
};
use anyhow::{bail, Context};
use craftflow_protocol_abstract::{
	c2s::{handshake::NextState, AbHandshake},
	s2c::AbLoginDisconnect,
	AbPacketNew, AbPacketWrite, AbS2C, State,
};
use craftflow_protocol_core::text;
use craftflow_protocol_versions::{MAX_VERSION, MIN_VERSION, S2C};
use reader::reader_task;
use std::{
	ops::ControlFlow,
	sync::{Arc, OnceLock, RwLock},
	time::Duration,
};
use tokio::{select, spawn, sync::mpsc::UnboundedReceiver, time::timeout};
use writer::writer_task;

/// The task that handles the connection and later splits into two tasks: reader and writer.
pub(super) async fn connection_task(
	craftflow: Arc<CraftFlow>,
	conn_id: u64,
	mut reader: PacketReader,
	mut writer: PacketWriter,
	concrete_packet_sender: UnboundedReceiver<S2C>,
	abstract_packet_sender: UnboundedReceiver<AbS2C>,
	reader_state: Arc<RwLock<State>>,
	writer_state: Arc<RwLock<State>>,
	protocol_version: Arc<OnceLock<u32>>,
	compression: Arc<OnceLock<usize>>,
	encryption_secret: Arc<OnceLock<[u8; 16]>>,
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

	// Ok so its NOT a legacy ping, lets continue with the normal handshake

	// we will read the handshake in this task before splitting into two tasks
	// so we know the next state for both tasks

	let mut handshake = match timeout(
		Duration::from_secs(5),
		reader.read_packet(&reader_state, MIN_VERSION, &compression, &mut None),
	)
	.await
	{
		Ok(p) => p.context("reading handshake packet")?,
		Err(_) => bail!("timed out"),
	};

	let handshake_ab = AbHandshake::construct(handshake.clone())?.assume_done();

	// set the client protocol version
	let client_version = handshake_ab.protocol_version;
	protocol_version
		.set(client_version)
		.expect("just got handshake but client protocol version already set");

	let next_state = match handshake_ab.next_state {
		NextState::Status => State::Status,
		NextState::Login | NextState::Transfer => State::Login,
	};
	*reader_state.write().unwrap() = next_state;
	*writer_state.write().unwrap() = next_state;

	// unless the next state is status, we need to check that the client protocol version is supported
	if handshake_ab.next_state != NextState::Status {
		if !(MIN_VERSION <= client_version && client_version <= MAX_VERSION) {
			let message = match craftflow
				.reactor
				.event::<UnsupportedClientVersion>(&craftflow, (conn_id, client_version))
			{
				ControlFlow::Continue(_) => {
					text!("Your version is not supported.", color = "white", bold)
				}
				ControlFlow::Break(message) => message,
			};

			writer
				.send(
					next_state,
					client_version,
					None,
					&mut None,
					&AbLoginDisconnect { message }
						.convert(client_version, State::Login)?
						.assume()
						.next()
						.unwrap(),
				)
				.await?;

			return Ok(()); // close the connection
		}
	}

	// trigger the handshake event
	if trigger_c2s_concrete(&craftflow, conn_id, &mut handshake).is_continue() {
		trigger_c2s_abstract(&craftflow, conn_id, &mut handshake_ab.into());
	}

	// now we can finally split into two tasks
	let cf_clone = Arc::clone(&craftflow);
	let reader_state_clone = Arc::clone(&reader_state);
	let compression_clone = Arc::clone(&compression);
	let encryption_secret_clone = Arc::clone(&encryption_secret);
	let reader_task = spawn(async move {
		reader_task(
			cf_clone,
			conn_id,
			client_version,
			reader,
			reader_state_clone,
			compression_clone,
			encryption_secret_clone,
		)
		.await
	});

	let writer_task = spawn(async move {
		writer_task(
			craftflow,
			conn_id,
			client_version,
			writer,
			concrete_packet_sender,
			abstract_packet_sender,
			reader_state,
			writer_state,
			compression,
			encryption_secret,
		)
		.await
	});

	select! {
		r = reader_task => r?.context("reader task"),
		r = writer_task => r?.context("writer task"),
	}
}
