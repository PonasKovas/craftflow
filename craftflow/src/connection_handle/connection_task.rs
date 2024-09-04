use super::{
	legacy::{detect_legacy_ping, write_legacy_response},
	packet_reader::PacketReader,
	packet_writer::PacketWriter,
	ConnState,
};
use crate::{
	packet_events::{trigger_c2s, trigger_s2c},
	CraftFlow,
};
use anyhow::{bail, Context};
use craftflow_protocol::packets::{
	handshake::{Handshake, HandshakeC2S, NextState},
	legacy::LegacyPing,
	login::LoginC2S,
	PacketC2S, PacketS2C,
};
use std::{
	ops::ControlFlow,
	sync::{Arc, OnceLock},
	time::Duration,
};
use tokio::{
	select, spawn,
	sync::mpsc::UnboundedReceiver,
	time::{sleep_until, timeout, Instant},
};

/// The task that handles the connection and later splits into two tasks: reader and writer.
pub(super) async fn connection_task(
	craftflow: Arc<CraftFlow>,
	conn_id: usize,
	mut reader: PacketReader,
	mut writer: PacketWriter,
	mut packet_sender: UnboundedReceiver<PacketS2C>,
	mut packet_batch_sender: UnboundedReceiver<Vec<PacketS2C>>,
	client_protocol_version: Arc<OnceLock<i32>>,
) -> anyhow::Result<()> {
	// First things first check if this is a legacy ping
	if let Some(legacy_ping_format) = detect_legacy_ping(&mut reader.stream).await? {
		// Trigger the legacy ping event
		if let ControlFlow::Break(()) = craftflow
			.reactor
			.event::<LegacyPing>(&craftflow.state, (conn_id, LegacyPing))
		{
			return Ok(());
		}

		// now if anyone did anything with the legacy ping event, they would have sent a response by now
		// so check both packet receivers
		let response = 'response: {
			// check normal channel
			loop {
				match packet_sender.try_recv() {
					Ok(PacketS2C::Legacy(packet)) => {
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
								PacketS2C::Legacy(packet) => break 'response packet,
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

	let handshake = match timeout(Duration::from_secs(5), reader.read_packet()).await {
		Ok(p) => p.context("reading packet")?,
		Err(_) => bail!("timed out"),
	};

	let handshake = match handshake {
		PacketC2S::HandshakeC2S(HandshakeC2S::Handshake(handshake)) => handshake,
		_ => unreachable!(), // the packet reader was in the handshake state so only handshake packets can be read
	};

	// set the client protocol version
	client_protocol_version
		.set(handshake.protocol_version.0)
		.expect("client protocol version already set");

	// trigger the handshake event
	let _ = craftflow
		.reactor
		.event::<Handshake>(&craftflow.state, (conn_id, handshake.clone()));

	// update the state of the reader and writer
	let state = match handshake.next_state {
		NextState::Status => ConnState::Status,
		NextState::Login | NextState::Transfer => ConnState::Login,
	};
	reader.state = state;
	writer.state = state;

	// now we can finally split into two tasks
	// spawn a task to handle writing packets
	// since we now know the state
	spawn(writer_task(
		Arc::clone(&craftflow),
		conn_id,
		writer,
		packet_sender,
		packet_batch_sender,
	));

	// continue reading packets in this task
	loop {
		let packet = reader.read_packet().await?;

		// match certain special packets that change the state
		match &packet {
			PacketC2S::LoginC2S(LoginC2S::LoginAcknowledged(_)) => {
				reader.state = ConnState::Configuration;
			}
			_ => {}
		}

		// trigger packet event
		trigger_c2s(&craftflow, conn_id, packet);
	}
}

/// The task that handles writing packets to the client.
pub(super) async fn writer_task(
	craftflow: Arc<CraftFlow>,
	conn_id: usize,
	mut writer: PacketWriter,
	mut packet_sender: UnboundedReceiver<PacketS2C>,
	mut packet_batch_sender: UnboundedReceiver<Vec<PacketS2C>>,
) -> anyhow::Result<()> {
	loop {
		select! {
			packet = packet_sender.recv() => {
				let packet = match packet {
					Some(p) => p,
					None => return Ok(()), // This means the connection has to be closed, as the handle was dropped
				};

				// trigger the packet event, and actually send it if it was not cancelled
				match trigger_s2c(&craftflow, conn_id, packet) {
					ControlFlow::Continue(packet) => {
						writer.send(packet).await?;
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
					match trigger_s2c(&craftflow, conn_id, packet) {
						ControlFlow::Continue(packet) => {
							writer.send(packet).await?;
						}
						ControlFlow::Break(()) => {}
					}
				}
			},
		}
	}
}
