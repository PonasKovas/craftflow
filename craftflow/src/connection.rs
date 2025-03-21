mod connection_task;
pub mod legacy;
mod packet_reader;
mod packet_writer;

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
	id: u64,
	ip: IpAddr,
	protocol_version: u32,
	packet_sender: Sender<S2C<'static>>,

	encryption_secret: Arc<OnceLock<[u8; 16]>>,
	compression: Arc<OnceLock<usize>>,

	// the state of the writing half of the connection.
	// almost in all cases this will be the same as the reading half
	writer_state: Arc<RwLock<State>>,
}

impl ConnectionInterface {
	/// Send an abstract packet to this client.
	pub async fn send(&self, packet: impl Into<AbS2C<'static>>) {
		if self
			.abstract_packet_sender
			.send(packet.into())
			.await
			.is_err()
		{
			error!("tried to send concrete packet but client writer task dead");
		}
	}
	/// Send a concrete packet to this client.
	pub async fn send_concrete(&self, packet: impl IntoStateEnum<Direction = S2C<'static>>) {
		if self
			.concrete_packet_sender
			.send(packet.into_state_enum())
			.await
			.is_err()
		{
			error!("tried to send abstract packet but client writer task dead");
		}
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
	pub fn id(&self) -> u64 {
		self.id
	}
}

impl Display for ConnectionInterface {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Connection[{}][{}]", self.id, self.ip)
	}
}
