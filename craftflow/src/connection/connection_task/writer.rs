use super::ConnectionInfo;
use crate::{
	connection::packet_writer::{Encryptor, PacketWriter},
	packet_events::{trigger_s2c_abstract, trigger_s2c_concrete},
	CraftFlow,
};
use aes::cipher::KeyIvInit;
use craftflow_protocol_abstract::{AbPacketWrite, AbS2C, State, WriteResult};
use craftflow_protocol_versions::{s2c, S2C};
use std::sync::{Arc, OnceLock};
use tokio::{select, sync::mpsc::Receiver};
use tracing::error;

/// The task that handles writing packets to the client.
pub(super) async fn writer_task(
	craftflow: Arc<CraftFlow>,
	mut writer: PacketWriter,
	mut concrete_packet_sender: Receiver<S2C<'static>>,
	mut abstract_packet_sender: Receiver<AbS2C<'static>>,
	conn: ConnectionInfo,
) -> anyhow::Result<()> {
	let mut encryptor = None;

	loop {
		select! {
			abs = abstract_packet_sender.recv() => {
				let abs = match abs {
					Some(p) => p,
					None => return Ok(()), // This means the connection has to be closed, as the handle was dropped
				};

				let (cont, abs) = trigger_s2c_abstract(false, &craftflow, conn.id, abs).await;
				if !cont {
					continue;
				}

				// convert the abstract packet to a series of concrete packets
				let state = *conn.writer_state.read().unwrap();
				let iter = match abs.convert(conn.version, state) {
					Ok(WriteResult::Success(iter)) => iter,
					Ok(WriteResult::Unsupported) => {
						error!(
							"Abstract packet {abs:?} not supported by this client (version {version}, state {state:?})",
							version = conn.version,
						);
						continue;
					}
					Err(e) => {
						error!(
							"Failed to convert packet {abs:?} (version {version}, state {state:?}): {}",
							e,
							version = conn.version,
						);
						continue;
					}
				};

				for concrete in iter {
					try_init_encryptor(&conn.encryption_secret, &mut encryptor);
					send_concrete(&craftflow, &mut writer, &conn, &mut encryptor, concrete).await?;
				}

				let _ = trigger_s2c_abstract(true, &craftflow, conn.id, abs);
			},
			concrete = concrete_packet_sender.recv() => {
				let concrete = match concrete {
					Some(p) => p,
					None => return Ok(()), // This means the connection has to be closed, as the handle was dropped
				};

				try_init_encryptor(&conn.encryption_secret, &mut encryptor);
				send_concrete(&craftflow, &mut writer, &conn, &mut encryptor, concrete).await?;
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
	writer: &mut PacketWriter,
	conn: &ConnectionInfo,
	encryptor: &mut Option<Encryptor>,
	packet: S2C<'a>,
) -> anyhow::Result<()> {
	// trigger the packet event, and actually send it if it was not cancelled
	let (cont, packet) = trigger_s2c_concrete(false, craftflow, conn.id, packet).await;
	if !cont {
		return Ok(());
	}

	// we check the state and compression before sending each packet individually
	// since any of the reactor events could change them
	let state = *conn.writer_state.read().unwrap();
	let compression = conn.compression.get().copied();
	writer
		.send(state, conn.version, compression, encryptor, &packet)
		.await?;

	// some special packets that change the state of the connection
	match packet {
		S2C::Login(s2c::Login::Success(_)) => {
			if conn.version >= 764 {
				// in this version acknowledgment packets were introduced and so
				// the states of the reader/writer separated
				// and also Configuration state was added
				*conn.writer_state.write().unwrap() = State::Configuration;
			} else {
				*conn.writer_state.write().unwrap() = State::Play;
				*conn.reader_state.write().unwrap() = State::Play;
			}
		}
		S2C::Configuration(s2c::Configuration::FinishConfiguration(_)) => {
			*conn.writer_state.write().unwrap() = State::Play;
		}
		_ => {}
	}

	let (_cont, _packet) = trigger_s2c_concrete(true, craftflow, conn.id, packet).await;

	Ok(())
}
