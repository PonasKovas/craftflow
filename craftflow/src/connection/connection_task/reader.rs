use crate::{
	connection::packet_reader::{Decryptor, PacketReader},
	packet_events::{trigger_c2s_abstract, trigger_c2s_concrete},
	CraftFlow,
};
use aes::cipher::KeyIvInit;
use anyhow::Context;
use craftflow_protocol_abstract::{
	AbC2S, AbPacketConstructor, AbPacketNew, ConcretePacket, ConstructorResult, State,
};
use craftflow_protocol_versions::{c2s, C2S};
use std::sync::Arc;
use tracing::error;

use super::ConnectionInfo;

type DynConstructor =
	Box<dyn AbPacketConstructor<AbPacket = AbC2S<'static>> + Send + Sync + 'static>;

pub(super) async fn reader_task(
	craftflow: Arc<CraftFlow>,
	mut reader: PacketReader,
	conn: ConnectionInfo,
) -> anyhow::Result<()> {
	let mut constructors: Vec<DynConstructor> = Vec::new();

	let mut decryptor = None;

	loop {
		if decryptor.is_none() {
			// check if encryption secret received
			if let Some(secret) = conn.encryption_secret.get() {
				decryptor = Some(Decryptor::new(secret.into(), secret.into()));
			}
		}

		let state = *conn.reader_state.read().unwrap();
		let packet = reader
			.read_packet(
				state,
				conn.version,
				conn.compression.get().copied(),
				&mut decryptor,
			)
			.await
			.with_context(|| format!("reading concrete packet (state {:?})", state))?;

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

		let (cont, packet) = trigger_c2s_concrete(false, &craftflow, conn.id, packet).await;

		// try to construct abstract packet
		let abstr = 'abstr: {
			// check all already started constructors
			for i in 0..constructors.len() {
				match constructors
					.get_mut(i)
					.unwrap()
					.next_packet(ConcretePacket::C2S(&packet))
					.context("abstract")?
				{
					ConstructorResult::Done(p) => {
						constructors.remove(i);
						break 'abstr p;
					}
					ConstructorResult::Continue(()) => {
						return Ok(());
					}
					ConstructorResult::Ignore => {}
				}
			}

			// Otherwise try constructing a new one
			match AbC2S::construct(&packet).context("abstract")? {
				ConstructorResult::Done(p) => break 'abstr p,
				ConstructorResult::Continue(c) => {
					constructors.push(c);
					return Ok(());
				}
				ConstructorResult::Ignore => {
					error!(
		                    "Failed to construct abstract packet from concrete packet: {:?} (This is most likely a bug within craftflow)",
		                    packet
		                );
					return Ok(());
				}
			}
		};

		if !cont {
			continue;
		}
		let (cont, abstr) = trigger_c2s_abstract(false, &craftflow, conn.id, abstr).await;
		if !cont {
			continue;
		}
		let (cont, _abstr) = trigger_c2s_abstract(true, &craftflow, conn.id, abstr).await;
		if !cont {
			continue;
		}
		let (_cont, _packet) = trigger_c2s_concrete(true, &craftflow, conn.id, packet).await;
	}
}
