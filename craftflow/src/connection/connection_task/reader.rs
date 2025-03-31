use super::{ConnectionInfo, trigger_c2s};
use crate::{
	CraftFlow,
	connection::{
		State,
		packet_reader::{Decryptor, PacketReader},
	},
};
use aes::cipher::KeyIvInit;
use anyhow::Context;
use craftflow_protocol::{C2S, c2s};
use std::sync::Arc;
use tracing::debug;

pub(super) async fn reader_task(
	craftflow: Arc<CraftFlow>,
	mut reader: PacketReader,
	conn: ConnectionInfo,
) -> anyhow::Result<()> {
	let mut decryptor = None;

	loop {
		if decryptor.is_none() {
			// check if encryption secret received
			if let Some(secret) = conn.encryption_secret.get() {
				decryptor = Some(Decryptor::new(secret.into(), secret.into()));
			}
		}

		let state = *conn.reader_state.read().unwrap();
		let result = reader
			.read_packet(state, conn.version, Some(&conn.compression), &mut decryptor)
			.await;

		// TODO when whole protocol implemented REMOVE THIS
		// but for now dont error if unknown packet received
		if let Err(e) = &result {
			if let Some(e) = e.downcast_ref::<craftflow_protocol::Error>() {
				if let craftflow_protocol::Error::UnknownPacketId {
					id: _,
					protocol_version: _,
					state: _,
				} = &e
				{
					debug!("received unknown packet {e}");
					continue;
				}
			}
		}

		let packet = result.with_context(|| format!("reading packet (state {:?})", state))?;

		// If None returned, that means the connection was cleanly closed on a packet boundary
		// in which case we dont want to print any errors
		let packet = match packet {
			Some(p) => p,
			None => return Ok(()),
		};

		// Handle some special packets which change the state of the connection
		match packet {
			C2S::Login(c2s::Login::LoginAcknowledged(_)) => {
				*conn.reader_state.write().unwrap() = State::Configuration;
			}
			C2S::Configuration(c2s::Configuration::FinishConfiguration(_)) => {
				*conn.reader_state.write().unwrap() = State::Play;
			}
			_ => {}
		}

		let (cont, packet) = trigger_c2s(false, &craftflow, conn.id, packet).await;
		if cont {
			trigger_c2s(true, &craftflow, conn.id, packet).await;
		}
	}
}
