mod compression;
mod connection_task;
mod encryption;
mod legacy;
mod packet_reader;
mod packet_writer;

use crate::CraftFlow;
use compression::CompressionSetter;
use connection_task::connection_task;
use craftflow_protocol::packets::{IntoPacketS2C, PacketS2C};
use encryption::EncryptionSetter;
use packet_reader::PacketReader;
use packet_writer::PacketWriter;
use std::{
	io::Cursor,
	sync::{Arc, OnceLock},
};
use tokio::{
	net::TcpStream,
	spawn,
	sync::mpsc::{self, UnboundedSender},
};
use tracing::error;

/// A handle to a client connection.
/// Use this to send packets or end the connection (by dropping this handle).
pub struct ConnectionHandle {
	pub(crate) packet_sender: UnboundedSender<PacketS2C>,
	/// For when you want to send multiple packets at once without anything in between them
	pub(crate) packet_batch_sender: UnboundedSender<Vec<PacketS2C>>,
	encryption: EncryptionSetter,
	compression: CompressionSetter,
	// the protocol version of the client
	// it is set by the reader task when handshake is received
	protocol_version: Arc<OnceLock<i32>>,
}

impl ConnectionHandle {
	/// Send a packet to this client.
	pub fn send(&self, packet: impl IntoPacketS2C) {
		// dont care if the client is disconnected
		let _ = self.packet_sender.send(packet.into_packet());
	}

	/// Send several packets to this client making sure nothing comes in-between
	pub fn send_batch(&self, packets: Vec<PacketS2C>) {
		// dont care if the client is disconnected
		let _ = self.packet_batch_sender.send(packets);
	}

	/// Set the encryption shared secret for this client.
	/// Make sure you send and handle the appropriate packets EncryptionRequest and EncryptionResponse
	/// this method has no safeguards.
	/// You can only set the encryption shared secret once.
	pub fn set_encryption(&self, shared_secret: [u8; 16]) {
		if let Err(()) = self.encryption.set(shared_secret) {
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
		if let Err(()) = self.compression.set(threshold) {
			error!("client compression threshold already set");
		}
	}

	/// Returns the protocol version of the client
	/// If handshake packet not received yet will return -1
	pub fn protocol_version(&self) -> i32 {
		self.protocol_version.get().copied().unwrap_or(-1)
	}
}

impl ConnectionHandle {
	/// Spawns the reading and writing tasks for a client connection.
	/// And adds the connection handle to the craftflow instance
	pub(crate) fn add(craftflow: &Arc<CraftFlow>, stream: TcpStream) {
		let (packet_sender_in, packet_sender_out) = mpsc::unbounded_channel();
		let (packet_batch_sender_in, packet_batch_sender_out) = mpsc::unbounded_channel();

		let (compression_setter, compression_getter1, compression_getter2) = compression::new();
		let (encryption_setter, encryptor, decryptor) = encryption::new();

		let (reader, writer) = stream.into_split();

		let packet_reader = PacketReader {
			stream: reader,
			buffer: Vec::with_capacity(1024 * 1024),
			state: ConnState::Handshake,
			decryptor,
			compression: compression_getter1,
		};
		let packet_writer = PacketWriter {
			stream: writer,
			buffer: Cursor::new(Vec::with_capacity(1024 * 1024)),
			state: ConnState::Handshake,
			encryptor,
			compression: compression_getter2,
		};

		let client_protocol_version = Arc::new(OnceLock::new());
		let client_protocol_version_clone = Arc::clone(&client_protocol_version);

		let handle = Self {
			packet_sender: packet_sender_in,
			packet_batch_sender: packet_batch_sender_in,
			encryption: encryption_setter,
			compression: compression_setter,
			protocol_version: client_protocol_version,
		};

		let conn_id = craftflow.state.connections.write().unwrap().insert(handle);

		let craftflow = Arc::clone(craftflow);
		spawn(async move {
			if let Err(e) = connection_task(
				Arc::clone(&craftflow),
				conn_id,
				packet_reader,
				packet_writer,
				packet_sender_out,
				packet_batch_sender_out,
				client_protocol_version_clone,
			)
			.await
			{
				// remove the connection from the list
				craftflow.state.connections.disconnect(conn_id);
				error!("Error handling connection: {:?}", e);
			}
		});
	}
}

// Used to track the state of the connection
#[derive(Debug, Clone, Copy, PartialEq)]
enum ConnState {
	Handshake,
	Status,
	Login,
	Configuration,
	Play,
}
