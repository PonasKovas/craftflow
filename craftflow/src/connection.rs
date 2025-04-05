mod common;
mod connection_task;
pub mod legacy;
mod packet_reader;
mod packet_writer;

use crate::ConnId;
use craftflow_protocol::S2C;
use std::{
	fmt::Display,
	net::IpAddr,
	sync::{Arc, OnceLock, RwLock},
};
use tokio::sync::mpsc::Sender;
use tracing::error;

pub(crate) use connection_task::handle_new_conn;

/// An interface to a client connection.
/// Use this to send packets or end the connection (by dropping this handle).
pub struct ConnectionInterface {
	id: ConnId,
	ip: IpAddr,
	protocol_version: u32,
	packet_sender: Sender<S2C>,

	encryption_secret: Arc<OnceLock<[u8; 16]>>,
	compression: Arc<OnceLock<usize>>,

	// the state of the writing half of the connection.
	// almost in all cases this will be the same as the reading half
	writer_state: Arc<RwLock<State>>,
}

/// Contains all the possible states of a connection
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum State {
	Handshake,
	Status,
	Login,
	Configuration,
	Play,
}

impl ConnectionInterface {
	/// Send a packet to this client.
	pub async fn send(&self, packet: impl Into<S2C>) {
		if self.packet_sender.send(packet.into()).await.is_err() {
			error!("tried to send abstract packet but client writer task dead");
		}
	}
	/// Set the encryption shared secret for this client.
	/// Make sure you send and handle the appropriate packets EncryptionRequest and EncryptionResponse
	/// this method has no safeguards.
	/// You can only set the encryption shared secret once.
	pub fn set_encryption(&self, shared_secret: [u8; 16]) {
		if self.encryption_secret.set(shared_secret).is_err() {
			error!("client encryption shared secret already set");
		}
	}
	/// Enables compression for this client with the given threshold.
	/// Make sure you send the appropriate packet SetCompression
	/// this method has no safeguards.
	/// You can only set the compression threshold once.
	///
	/// The threshold is the size of packet in bytes at which the server will start compressing it.
	pub fn set_compression_threshold(&self, threshold: usize) {
		if self.compression.set(threshold).is_err() {
			error!("client compression threshold already set");
		}
	}
	/// Returns the protocol version of the client
	pub fn protocol_version(&self) -> u32 {
		self.protocol_version
	}
	/// Returns the state of the connection
	pub fn state(&self) -> State {
		*self.writer_state.read().unwrap()
	}
	/// Returns the ip address of the client
	pub fn ip(&self) -> IpAddr {
		self.ip
	}
	/// Returns the ID of the connection
	pub fn id(&self) -> ConnId {
		self.id
	}
}

impl Display for ConnectionInterface {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Connection[{}][{}]", self.id, self.ip)
	}
}
