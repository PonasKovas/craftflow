mod add;
mod connection_task;
pub mod legacy;
mod packet_reader;
mod packet_writer;

use craftflow_protocol_abstract::{AbS2C, State};
use craftflow_protocol_versions::{IntoStateEnum, S2C};
use std::{
	fmt::Display,
	net::IpAddr,
	sync::{Arc, OnceLock, RwLock},
};
use tokio::sync::mpsc::UnboundedSender;
use tracing::error;

pub(crate) use add::new_conn_interface;

/// An interface to a client connection.
/// Use this to send packets or end the connection (by dropping this handle).
pub struct ConnectionInterface {
	id: u64,
	ip: IpAddr,
	concrete_packet_sender: UnboundedSender<S2C>,
	abstract_packet_sender: UnboundedSender<AbS2C>,

	encryption_secret: Arc<OnceLock<[u8; 16]>>,
	compression: Arc<OnceLock<usize>>,
	// the protocol version of the client
	// it is set by the reader task when handshake is received
	protocol_version: Arc<OnceLock<u32>>,
	// the state of the writing half of the connection.
	// almost in all cases this will be the same as the reading half
	writer_state: Arc<RwLock<State>>,
}

impl ConnectionInterface {
	/// Send an abstract packet to this client.
	pub fn send(&self, packet: impl Into<AbS2C>) {
		let _ = self.abstract_packet_sender.send(packet.into());
	}
	/// Send a concrete packet to this client.
	pub fn send_concrete(&self, packet: impl IntoStateEnum<Direction = S2C>) {
		// dont care if the client is disconnected
		let _ = self.concrete_packet_sender.send(packet.into_state_enum());
	}
	/// Set the encryption shared secret for this client.
	/// Make sure you send and handle the appropriate packets EncryptionRequest and EncryptionResponse
	/// this method has no safeguards.
	/// You can only set the encryption shared secret once.
	pub fn set_encryption(&self, shared_secret: [u8; 16]) {
		if let Err(_) = self.encryption_secret.set(shared_secret) {
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
		if let Err(_) = self.compression.set(threshold) {
			error!("client compression threshold already set");
		}
	}
	/// Returns the protocol version of the client
	/// If handshake packet not received yet will return 0
	pub fn protocol_version(&self) -> u32 {
		self.protocol_version.get().copied().unwrap_or(0)
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
	pub fn id(&self) -> u64 {
		self.id
	}
}

impl Display for ConnectionInterface {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Connection[{}][{}]", self.id, self.ip)
	}
}
