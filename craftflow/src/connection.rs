mod add;
mod connection_task;
pub mod legacy;
mod packet_reader;
mod packet_writer;

use crate::{packet_events::trigger_s2c_abstract_pre, CraftFlow};
use connection_task::connection_task;
use craftflow_protocol_abstract::{AbPacketWrite, AbS2C, WriteResult};
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
	craftflow: Arc<CraftFlow>,
	id: u64,
	ip: IpAddr,
	// This is put in RwLock to allow threads to send multiple packets without anything in between
	// from other threads, by requesting exclusive access to the sender.
	packet_sender: RwLock<UnboundedSender<S2C>>,

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
	craftflow: &'a CraftFlow,
	id: u64,
	version: u32,
	lock: std::sync::RwLockWriteGuard<'a, UnboundedSender<S2C>>,
}

impl<'a> PacketBatchSender<'a> {
	/// Send an abstract packet to this client.
	/// Ignores errors when converting, just logs them
	pub fn send(&self, packet: impl Into<AbS2C>) -> WriteResult<()> {
		let mut packet = packet.into();

		trigger_s2c_abstract_pre(self.craftflow, self.id, &mut packet);

		// convert the abstract packet to a series of concrete packets
		let iter = match packet.convert(self.version) {
			Ok(WriteResult::Success(iter)) => iter,
			Ok(WriteResult::Unsupported) => {
				return WriteResult::Unsupported;
			}
			Err(e) => {
				error!("Failed to convert packet: {}", e);
				return WriteResult::Success(());
			}
		};

		for concrete in iter {
			// dont care if the client is disconnected
			let _ = self.lock.send(concrete);
		}

		WriteResult::Success(())
	}
	/// Send a concrete packet to this client.
	pub fn send_concrete(&self, packet: impl IntoStateEnum<Direction = S2C>) -> &Self {
		// dont care if the client is disconnected
		let _ = self.lock.send(packet.into_state_enum());

		self
	}
}

impl ConnectionHandle {
	/// Send an abstract packet to this client.
	/// Ignores errors when converting, just logs them
	pub fn send(&self, packet: impl Into<AbS2C>) -> WriteResult<()> {
		let mut packet = packet.into();

		trigger_s2c_abstract_pre(&self.craftflow, self.id, &mut packet);

		// convert the abstract packet to a series of concrete packets
		let iter = match packet.convert(self.protocol_version()) {
			Ok(WriteResult::Success(iter)) => iter,
			Ok(WriteResult::Unsupported) => {
				return WriteResult::Unsupported;
			}
			Err(e) => {
				error!("Failed to convert packet: {}", e);
				return WriteResult::Success(());
			}
		};

		let lock = self.packet_sender.read().unwrap();
		for concrete in iter {
			// dont care if the client is disconnected
			let _ = lock.send(concrete);
		}

		WriteResult::Success(())
	}
	/// Send a concrete packet to this client.
	pub fn send_concrete(&self, packet: impl IntoStateEnum<Direction = S2C>) {
		// dont care if the client is disconnected
		let _ = self
			.packet_sender
			.read()
			.unwrap()
			.send(packet.into_state_enum());
	}
	/// Send several packets to this client making sure nothing comes in-between
	pub fn batch_sender(&self) -> PacketBatchSender {
		let lock = self.packet_sender.write().unwrap();
		PacketBatchSender {
			version: self.protocol_version(),
			lock,
			craftflow: &self.craftflow,
			id: self.id,
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
	/// If handshake packet not received yet will panic
	pub fn protocol_version(&self) -> u32 {
		self.protocol_version
			.get()
			.copied()
			.expect("handshake not received yet")
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
