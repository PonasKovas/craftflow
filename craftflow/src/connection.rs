mod add;
mod connection_task;
pub mod legacy;
mod packet_reader;
mod packet_writer;

use crate::packets::S2CPacket;
use connection_task::connection_task;
use craftflow_protocol_abstract::AbS2C;
use craftflow_protocol_versions::{IntoStateEnum, S2C};
use std::{
	fmt::Display,
	net::IpAddr,
	sync::{Arc, OnceLock, RwLock},
};
use tokio::sync::mpsc::UnboundedSender;
use tracing::error;

/// A handle to a client connection.
/// Use this to send packets or end the connection (by dropping this handle).
pub struct ConnectionHandle {
	id: u64,
	ip: IpAddr,
	// This is put in RwLock to allow threads to send multiple packets without anything in between
	// from other threads, by requesting exclusive access to the sender.
	packet_sender: RwLock<UnboundedSender<S2CPacket>>,

	encryption_secret: Arc<OnceLock<[u8; 16]>>,
	compression: Arc<OnceLock<usize>>,
	// the protocol version of the client
	// it is set by the reader task when handshake is received
	protocol_version: Arc<OnceLock<u32>>,
	// the state of the connection. Certain packets change this
	state: Arc<RwLock<ConnState>>,
}

// Used to track the state of the connection
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConnState {
	Handshake,
	Status,
	Login,
	Configuration,
	Play,
}

/// Guarantees that packets are sent in a row without any other packets in between them
pub struct PacketBatchSender<'a> {
	lock: std::sync::RwLockWriteGuard<'a, UnboundedSender<S2CPacket>>,
}

impl<'a> PacketBatchSender<'a> {
	/// Send an abstract packet to this client.
	pub fn send(&self, packet: impl Into<AbS2C>) -> &Self {
		// dont care if the client is disconnected
		let _ = self.lock.send(S2CPacket::Abstract(packet.into()));

		self
	}
	/// Send a concrete packet to this client.
	pub fn send_concrete(&self, packet: impl IntoStateEnum<Direction = S2C>) -> &Self {
		// dont care if the client is disconnected
		let _ = self
			.lock
			.send(S2CPacket::Concrete(packet.into_state_enum()));

		self
	}
}

impl ConnectionHandle {
	/// Send an abstract packet to this client.
	pub fn send(&self, packet: impl Into<AbS2C>) {
		// dont care if the client is disconnected
		let _ = self
			.packet_sender
			.read()
			.unwrap()
			.send(S2CPacket::Abstract(packet.into()));
	}
	/// Send a concrete packet to this client.
	pub fn send_concrete(&self, packet: impl IntoStateEnum<Direction = S2C>) {
		// dont care if the client is disconnected
		let _ = self
			.packet_sender
			.read()
			.unwrap()
			.send(S2CPacket::Concrete(packet.into_state_enum()));
	}
	/// Send several packets to this client making sure nothing comes in-between
	pub fn batch_sender(&self) -> PacketBatchSender {
		let lock = self.packet_sender.write().unwrap();
		PacketBatchSender { lock }
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
	/// If handshake packet not received yet will panic
	pub fn protocol_version(&self) -> u32 {
		self.protocol_version.get().copied().expect("handshake not received yet and you already want to know protocol version WTF is wrong with you")
	}
	/// Returns the state of the connection
	pub fn state(&self) -> ConnState {
		*self.state.read().unwrap()
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

impl Display for ConnectionHandle {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Connection[{}][{}]", self.id, self.ip)
	}
}
