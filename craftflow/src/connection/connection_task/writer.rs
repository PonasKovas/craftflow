use super::ConnectionInfo;
use crate::{
	CraftFlow,
	connection::{
		State,
		packet_writer::{Encryptor, PacketWriter},
	},
	packet_events::trigger_s2c,
};
use aes::cipher::KeyIvInit;
use craftflow_protocol::{S2C, s2c};
use std::sync::{Arc, OnceLock};
use tokio::sync::mpsc::Receiver;

/// The task that handles writing packets to the client.
pub(super) async fn writer_task(
	craftflow: Arc<CraftFlow>,
	mut writer: PacketWriter,
	mut packet_sender: Receiver<S2C>,
	conn: ConnectionInfo,
) -> anyhow::Result<()> {
	let mut encryptor = None;

	loop {
		let packet = match packet_sender.recv().await {
			Some(p) => p,
			None => return Ok(()), // This means the connection has to be closed, as the handle was dropped
		};

		try_init_encryptor(&conn.encryption_secret, &mut encryptor);
		send(&craftflow, &mut writer, &conn, &mut encryptor, packet).await?;
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

async fn send(
	craftflow: &CraftFlow,
	writer: &mut PacketWriter,
	conn: &ConnectionInfo,
	encryptor: &mut Option<Encryptor>,
	packet: S2C,
) -> anyhow::Result<()> {
	// trigger the packet event, and actually send it if it was not cancelled
	let (cont, packet) = trigger_s2c(false, craftflow, conn.id, packet).await;
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

	trigger_s2c(true, craftflow, conn.id, packet).await;

	Ok(())
}
