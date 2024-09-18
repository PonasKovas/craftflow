use super::{
	legacy::{detect_legacy_ping, write_legacy_response},
	packet_reader::PacketReader,
	packet_writer::PacketWriter,
	ConnState,
};
use crate::{
	packet_events::{trigger_c2s, trigger_s2c_post, trigger_s2c_pre},
	CraftFlow,
};
use anyhow::{bail, Context};
use craftflow_protocol::{
	legacy::LegacyPing,
	protocol::{
		c2s::{
			handshake::{Handshake, NextState},
			HandshakePacket,
		},
		C2S, S2C,
	},
};
use std::{
	ops::ControlFlow,
	sync::{Arc, OnceLock},
	time::Duration,
};
use tokio::{select, spawn, sync::mpsc::UnboundedReceiver, time::timeout};
use tracing::error;

/// The task that handles the connection and later splits into two tasks: reader and writer.
pub(super) async fn connection_task(
	craftflow: Arc<CraftFlow>,
	conn_id: usize,
	mut reader: PacketReader,
	mut writer: PacketWriter,
	mut packet_sender: UnboundedReceiver<S2C>,
	mut packet_batch_sender: UnboundedReceiver<Vec<S2C>>,
	client_protocol_version: Arc<OnceLock<u32>>,
) -> anyhow::Result<()> {
	// First things first check if this is a legacy ping
	if let Some(legacy_ping_format) = detect_legacy_ping(&mut reader.stream).await? {
		// Trigger the legacy ping event
		if let ControlFlow::Break(()) = craftflow
			.reactor
			.event::<LegacyPing>(&craftflow, (conn_id, LegacyPing))
		{
			return Ok(());
		}

		// now if anyone did anything with the legacy ping event, they would have sent a response
		// so check both packet receivers
		let response = 'response: {
			// check normal channel
			loop {
				match packet_sender.try_recv() {
					Ok(S2C::LegacyPingResponse(packet)) => {
						break 'response packet;
					}
					Ok(_) => continue,
					Err(_) => break,
				}
			}
			// check batch channel
			loop {
				match packet_batch_sender.try_recv() {
					Ok(batch) => {
						for packet in batch {
							match packet {
								S2C::LegacyPingResponse(packet) => break 'response packet,
								_ => continue,
							}
						}
					}
					Err(_) => break,
				}
			}

			// no response found :/
			return Ok(()); // close the connection
		};

		write_legacy_response(&mut writer.stream, legacy_ping_format, response).await?;
		return Ok(()); // close the connection
	}

	// Ok so its not a legacy ping, lets continue with the normal handshake

	// we will read the handshake in this task before splitting into two tasks
	// so we know the next state for both tasks
	let next_state = {
		let handshake = match timeout(Duration::from_secs(5), reader.read_packet()).await {
			Ok(p) => p.context("reading handshake packet")?,
			Err(_) => bail!("timed out"),
		};

		let handshake = match handshake {
			C2S::Handshake(HandshakePacket::Handshake { packet }) => packet,
			_ => unreachable!(), // the packet reader was in the handshake state so only handshake packets can be read
		};

		// check and set the client protocol version
		let protocol_version = handshake.protocol_version.0 as u32;
		// unless the next_state is status, the protocol version must be supported
		if !craftflow_protocol::protocol::SUPPORTED_PROTOCOL_VERSIONS.contains(&protocol_version)
			&& handshake.next_state != (NextState::Status {})
		{
			bail!("unsupported protocol version");
		}
		client_protocol_version
			.set(protocol_version)
			.expect("client protocol version already set");

		// trigger the handshake event
		let _ = craftflow
			.reactor
			.event::<Handshake>(&craftflow, (conn_id, handshake.clone()));

		match handshake.next_state {
			NextState::Status {} => ConnState::Status,
			NextState::Login {} | NextState::Transfer {} => ConnState::Login,
			NextState::_Unsupported => bail!("unsupported next state"),
		}
	};

	// update the state of the reader and writer
	reader.state = next_state;
	writer.state = next_state;

	// now we can finally split into two tasks
	// spawn a task to handle writing packets
	// since we now know the state
	let cf_clone = Arc::clone(&craftflow);
	spawn(async move {
		if let Err(e) = writer_task(
			cf_clone,
			conn_id,
			writer,
			packet_sender,
			packet_batch_sender,
		)
		.await
		{
			error!("writer task: {e}");
		}
	});

	// continue reading packets in this task
	if let Err(e) = reader_task(craftflow, conn_id, reader).await {
		error!("reader task: {e}");
	}

	Ok(())
}

/// The task that handles writing packets to the client.
pub(super) async fn writer_task(
	craftflow: Arc<CraftFlow>,
	conn_id: usize,
	mut writer: PacketWriter,
	mut packet_sender: UnboundedReceiver<S2C>,
	mut packet_batch_sender: UnboundedReceiver<Vec<S2C>>,
) -> anyhow::Result<()> {
	loop {
		select! {
			packet = packet_sender.recv() => {
				let packet = match packet {
					Some(p) => p,
					None => return Ok(()), // This means the connection has to be closed, as the handle was dropped
				};

				// trigger the packet event, and actually send it if it was not cancelled
				match trigger_s2c_pre(&craftflow, conn_id, packet) {
					ControlFlow::Continue(packet) => {
						writer.send(&packet).await?;
						let _ = trigger_s2c_post(&craftflow, conn_id, packet);
					}
					ControlFlow::Break(()) => {}
				}
			},
			batch = packet_batch_sender.recv() => {
				let batch = match batch {
					Some(b) => b,
					None => return Ok(()), // This means the connection has to be closed, as the handle was dropped
				};

				for packet in batch {
					// trigger the packet event, and actually send it if it was not cancelled
					match trigger_s2c_pre(&craftflow, conn_id, packet) {
						ControlFlow::Continue(packet) => {
							writer.send(&packet).await?;
							let _ = trigger_s2c_post(&craftflow, conn_id, packet);
						}
						ControlFlow::Break(()) => {}
					}
				}
			},
		}
	}
}

async fn reader_task(
	craftflow: Arc<CraftFlow>,
	conn_id: usize,
	mut reader: PacketReader,
) -> anyhow::Result<()> {
	loop {
		let packet = reader.read_packet().await?;

		// trigger packet event
		trigger_c2s(&craftflow, conn_id, packet);
	}
}
