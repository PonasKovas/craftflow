mod compression;
mod connection_task;
mod encryption;
mod legacy;
mod packet_reader;
mod packet_writer;

use crate::CraftFlow;
use compression::CompressionSetter;
use connection_task::connection_task;
use craftflow_protocol::{protocol::S2C, Packet};
use encryption::EncryptionSetter;
use futures::FutureExt;
use packet_reader::PacketReader;
use packet_writer::PacketWriter;
use std::{
	fmt::Display,
	io::Cursor,
	net::IpAddr,
	panic::AssertUnwindSafe,
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
	id: usize,
	ip: IpAddr,
	pub(crate) packet_sender: UnboundedSender<S2C>,
	/// For when you want to send multiple packets at once without anything in between them
	pub(crate) packet_batch_sender: UnboundedSender<Vec<S2C>>,
	encryption: EncryptionSetter,
	compression: CompressionSetter,
	// the protocol version of the client
	// it is set by the reader task when handshake is received
	protocol_version: Arc<OnceLock<u32>>,
}

impl ConnectionHandle {
	/// Send a packet to this client.
	pub fn send(&self, packet: impl Packet<Direction = S2C>) {
		// dont care if the client is disconnected
		let _ = self.packet_sender.send(packet.into_packet_enum());
	}

	/// Send several packets to this client making sure nothing comes in-between
	pub fn send_batch(&self, packets: Vec<S2C>) {
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
	/// If handshake packet not received yet will panic
	pub fn protocol_version(&self) -> u32 {
		self.protocol_version.get().copied().expect("handshake not received yet and you already want to know protocol version WTF is wrong with you")
	}

	/// Returns the ip address of the client
	pub fn ip(&self) -> IpAddr {
		self.ip
	}

	/// Returns the ID of the connection
	pub fn id(&self) -> usize {
		self.id
	}
}

impl ConnectionHandle {
	/// Spawns the reading and writing tasks for a client connection.
	/// And adds the connection handle to the craftflow instance
	/// returns the ID of the connection
	pub(crate) fn add(craftflow: &Arc<CraftFlow>, stream: TcpStream) -> usize {
		let peer_ip = stream.peer_addr().unwrap().ip();

		let (packet_sender_in, packet_sender_out) = mpsc::unbounded_channel();
		let (packet_batch_sender_in, packet_batch_sender_out) = mpsc::unbounded_channel();

		let (compression_setter, compression_getter1, compression_getter2) = compression::new();
		let (encryption_setter, encryptor, decryptor) = encryption::new();

		let (reader, writer) = stream.into_split();

		let client_protocol_version = Arc::new(OnceLock::new());

		let packet_reader = PacketReader {
			stream: reader,
			buffer: Vec::with_capacity(1024 * 1024),
			state: ConnState::Handshake,
			decryptor,
			compression: compression_getter1,
			protocol_version: Arc::clone(&client_protocol_version),
		};
		let packet_writer = PacketWriter {
			stream: writer,
			buffer: Cursor::new(Vec::with_capacity(1024 * 1024)),
			state: ConnState::Handshake,
			encryptor,
			compression: compression_getter2,
			protocol_version: Arc::clone(&client_protocol_version),
		};

		let client_protocol_version_clone = Arc::clone(&client_protocol_version);

		let handle = Self {
			id: 0, // set below
			ip: peer_ip,
			packet_sender: packet_sender_in,
			packet_batch_sender: packet_batch_sender_in,
			encryption: encryption_setter,
			compression: compression_setter,
			protocol_version: client_protocol_version,
		};

		// Insert into the connections slab
		let conn_id = {
			let mut lock = craftflow.connections.write().unwrap();
			let conn_id = lock.insert(handle);
			lock[conn_id].id = conn_id;
			conn_id
		};

		let craftflow = Arc::clone(craftflow);
		spawn(async move {
			// Fuck you and your unwind safety.
			// i wont be accessing any of the state of this future,
			// i just need to know if it panicked
			let r = AssertUnwindSafe(connection_task(
				Arc::clone(&craftflow),
				conn_id,
				packet_reader,
				packet_writer,
				packet_sender_out,
				packet_batch_sender_out,
				client_protocol_version_clone,
			))
			.catch_unwind() // generally this shouldnt panic, but if it does, we still want to remove the connection
			.await;

			match r {
				Ok(Ok(_)) => {} // ended peacefully 😊
				Ok(Err(e)) => {
					error!("Error handling connection: {e:?}");
				}
				Err(_) => {} // panicked... wow.. cringe
			}

			// remove the connection from the list
			craftflow.disconnect(conn_id);
		});

		conn_id
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

impl Display for ConnectionHandle {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Connection[{}][{}]", self.id, self.ip)
	}
}
