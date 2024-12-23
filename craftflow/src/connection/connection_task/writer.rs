use crate::{
	connection::packet_writer::{Encryptor, PacketWriter},
	packet_events::{trigger_s2c_abstract, trigger_s2c_concrete},
	CraftFlow,
};
use aes::cipher::KeyIvInit;
use craftflow_protocol_abstract::{AbPacketWrite, AbS2C, State, WriteResult};
use craftflow_protocol_versions::{s2c, S2C};
use std::sync::{Arc, OnceLock, RwLock};
use tokio::{select, sync::mpsc::UnboundedReceiver};
use tracing::error;

/// The task that handles writing packets to the client.
pub(super) async fn writer_task(
	craftflow: Arc<CraftFlow>,
	conn_id: u64,
	version: u32,
	mut writer: PacketWriter,
	mut concrete_packet_sender: UnboundedReceiver<S2C<'static>>,
	mut abstract_packet_sender: UnboundedReceiver<AbS2C<'static>>,
	reader_state: Arc<RwLock<State>>,
	writer_state: Arc<RwLock<State>>,
	compression: Arc<OnceLock<usize>>,
	encryption_secret: Arc<OnceLock<[u8; 16]>>,
) -> anyhow::Result<()> {
	let mut encryptor = None;

	loop {
		select! {
			abs = abstract_packet_sender.recv() => {
				let abs = match abs {
					Some(p) => p,
					None => return Ok(()), // This means the connection has to be closed, as the handle was dropped
				};

				let (cont, abs) = trigger_s2c_abstract(false, &craftflow, conn_id, abs).await;
				if !cont {
					continue;
				}

				// convert the abstract packet to a series of concrete packets
				let state = *writer_state.read().unwrap();
				let iter = match abs.convert(version, state) {
					Ok(WriteResult::Success(iter)) => iter,
					Ok(WriteResult::Unsupported) => {
						error!("Abstract packet {abs:?} not supported by this client (version {version}, state {state:?})");
						continue;
					}
					Err(e) => {
						error!("Failed to convert packet {abs:?} (version {version}, state {state:?}): {}", e);
						continue;
					}
				};

				for concrete in iter {
					try_init_encryptor(&encryption_secret, &mut encryptor);
					send_concrete(&craftflow, conn_id, version, &mut writer, &reader_state, &writer_state, &compression, &mut encryptor, concrete).await?;
				}

				let _ = trigger_s2c_abstract(true, &craftflow, conn_id, abs);
			},
			concrete = concrete_packet_sender.recv() => {
				let concrete = match concrete {
					Some(p) => p,
					None => return Ok(()), // This means the connection has to be closed, as the handle was dropped
				};

				try_init_encryptor(&encryption_secret, &mut encryptor);
				send_concrete(&craftflow, conn_id, version, &mut writer, &reader_state, &writer_state, &compression, &mut encryptor, concrete).await?;
			},
		}
	}
}

// Checks if the secret is set yet and initializes the encryptor if it is
fn try_init_encryptor(encryption_secret: &OnceLock<[u8; 16]>, encryptor: &mut Option<Encryptor>) {
	if encryptor.is_none() {
		if let Some(secret) = encryption_secret.get() {
			*encryptor = Some(Encryptor::new(secret.into(), secret.into()));
		}
	}
}

async fn send_concrete<'a>(
	craftflow: &CraftFlow,
	conn_id: u64,
	version: u32,
	writer: &mut PacketWriter,
	reader_state: &RwLock<State>,
	writer_state: &RwLock<State>,
	compression: &OnceLock<usize>,
	encryptor: &mut Option<Encryptor>,
	packet: S2C<'a>,
) -> anyhow::Result<()> {
	// trigger the packet event, and actually send it if it was not cancelled
	let (cont, packet) = trigger_s2c_concrete(false, craftflow, conn_id, packet).await;
	if !cont {
		return Ok(());
	}

	// we check the state and compression before sending each packet individually
	// since any of the reactor events could change them
	let state = *writer_state.read().unwrap();
	let compression = compression.get().copied();
	writer
		.send(state, version, compression, encryptor, &packet)
		.await?;

	// some special packets that change the state of the connection
	match packet {
		S2C::Status(s2c::Status::Ping(_)) => {
			craftflow.disconnect(conn_id).await; // todo i dont think this is the right place to do this
			                            // try doing this in the ping module instead
		}
		S2C::Login(s2c::Login::Success(_)) => {
			if version >= 764 {
				// in this version acknowledgment packets were introduced and so
				// the states of the reader/writer separated
				// and also Configuration state was added
				*writer_state.write().unwrap() = State::Configuration;
			} else {
				*writer_state.write().unwrap() = State::Play;
				*reader_state.write().unwrap() = State::Play;
			}
		}
		S2C::Configuration(s2c::Configuration::FinishConfiguration(_)) => {
			*writer_state.write().unwrap() = State::Play;
		}
		_ => {}
	}

	let (_cont, _packet) = trigger_s2c_concrete(true, craftflow, conn_id, packet).await;

	Ok(())
}
