mod compression;
mod encryption;
mod legacy;
mod reader;
mod writer;

use anyhow::{bail, Context};
use compression::Compression;
use craftflow_protocol::packets::{
	handshake::{HandshakeC2S, NextState},
	legacy::LegacyPing,
	login::LoginC2S,
	IntoPacketC2S, IntoPacketS2C, PacketC2S, PacketS2C,
};
use encryption::{Decryptor, Encryptor};
use legacy::{detect_legacy_ping, write_legacy_response};
use reader::PacketReader;
use std::{
	io::Cursor,
	sync::{Arc, OnceLock},
	time::Duration,
};
use tokio::{
	net::TcpStream,
	select, spawn,
	sync::{
		mpsc::{self, Receiver, Sender, UnboundedReceiver, UnboundedSender},
		oneshot,
	},
	time::timeout,
};
use tracing::error;
use writer::PacketWriter;

/// A handle to a client connection.
/// Use this to send/receive packets or end the connection (by dropping this handle).
pub struct ConnectionHandle {
	pub(crate) packet_sender: UnboundedSender<PacketS2C>,
	/// For when you want to send multiple packets at once without anything in between
	packet_batch_sender: UnboundedSender<Vec<PacketS2C>>,
	pub(crate) packet_receiver: Receiver<PacketC2S>,
	// these are in pairs because there's one for the writer and one for the reader tasks
	// [reader, writer]
	set_encryption_shared_secret: Option<[oneshot::Sender<[u8; 16]>; 2]>,
	set_compression_threshold: Option<[oneshot::Sender<usize>; 2]>,
	// the protocol version of the client
	// it is set by the reader task when handshake is received
	protocol_version: Arc<OnceLock<i32>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConnState {
	Handshake,
	Status,
	Login,
	Configuration,
	Play,
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
	pub fn set_encryption_shared_secret(&mut self, shared_secret: [u8; 16]) {
		match self.set_encryption_shared_secret.take() {
			Some(senders) => {
				for sender in senders {
					let _ = sender.send(shared_secret);
				}
			}
			None => {
				error!("attempt to set encryption shared secret on a client more than once");
			}
		}
	}

	/// Enables compression for this client with the given threshold.
	/// Make sure you send the appropriate packet SetCompression
	/// this method has no safeguards.
	/// You can only set the compression threshold once.
	///
	/// The threshold is the size of packet in bytes at which the server will start compressing it.
	pub fn set_compression_threshold(&mut self, threshold: usize) {
		match self.set_compression_threshold.take() {
			Some(senders) => {
				for sender in senders {
					let _ = sender.send(threshold);
				}
			}
			None => {
				error!("attempt to set compression threshold on a client more than once");
			}
		}
	}

	/// Returns the protocol version of the client
	/// If handshake packet not received yet will return -1
	pub fn protocol_version(&self) -> i32 {
		self.protocol_version.get().copied().unwrap_or(-1)
	}
}

impl ConnectionHandle {
	pub(crate) fn new(stream: TcpStream) -> Self {
		let (packet_sender_in, packet_sender_out) = mpsc::unbounded_channel();
		let (packet_batch_sender_in, packet_batch_sender_out) = mpsc::unbounded_channel();
		let (packet_receiver_in, packet_receiver_out) = mpsc::channel(16);

		let (reader_encryption_secret_in, reader_encryption_secret_out) = oneshot::channel();
		let (writer_encryption_secret_in, writer_encryption_secret_out) = oneshot::channel();

		let (reader_compression_in, reader_compression_out) = oneshot::channel();
		let (writer_compression_in, writer_compression_out) = oneshot::channel();

		let (reader, writer) = stream.into_split();

		let packet_reader = PacketReader {
			stream: reader,
			buffer: Vec::with_capacity(1024 * 1024),
			state: ConnState::Handshake,
			decryptor: Decryptor::new(reader_encryption_secret_out),
			compression: Compression::new(reader_compression_out),
		};
		let packet_writer = PacketWriter {
			stream: writer,
			buffer: Cursor::new(Vec::with_capacity(1024 * 1024)),
			state: ConnState::Handshake,
			encryptor: Encryptor::new(writer_encryption_secret_out),
			compression: Compression::new(writer_compression_out),
		};

		let client_protocol_version = Arc::new(OnceLock::new());
		let client_protocol_version_clone = Arc::clone(&client_protocol_version);

		spawn(async move {
			if let Err(e) = connection_task(
				packet_reader,
				packet_writer,
				packet_sender_out,
				packet_batch_sender_out,
				packet_receiver_in,
				client_protocol_version_clone,
			)
			.await
			{
				error!("Error handling connection: {:?}", e);
			}
		});

		Self {
			packet_sender: packet_sender_in,
			packet_batch_sender: packet_batch_sender_in,
			packet_receiver: packet_receiver_out,
			set_encryption_shared_secret: Some([
				reader_encryption_secret_in,
				writer_encryption_secret_in,
			]),
			set_compression_threshold: Some([reader_compression_in, writer_compression_in]),
			protocol_version: client_protocol_version,
		}
	}
}

/// The task that handles the connection and later splits into two tasks: reader and writer.
async fn connection_task(
	mut reader: PacketReader,
	mut writer: PacketWriter,
	mut packet_sender: UnboundedReceiver<PacketS2C>,
	mut packet_batch_sender: UnboundedReceiver<Vec<PacketS2C>>,
	packet_receiver: Sender<PacketC2S>,
	client_protocol_version: Arc<OnceLock<i32>>,
) -> anyhow::Result<()> {
	// First things first check if this is a legacy ping
	if let Some(legacy_ping_format) = detect_legacy_ping(&mut reader.stream).await? {
		packet_receiver.send(LegacyPing.into_packet()).await?;

		// since nothing will be read from the client anymore
		// we can use this task to handle writing packets
		// and when I say packets I mean the legacy ping response
		let response = 'outer: loop {
			// skip all packets until we hopefully get the legacy ping response
			select! {
				packet = packet_sender.recv() => {
					match packet {
						Some(PacketS2C::Legacy(response)) => break 'outer response,
						Some(_) => continue, // some other packet, we only need LegacyPingResponse
						None => return Ok(()), // This means the connection has to be closed, as the handle was dropped
					}
				}
				batch = packet_batch_sender.recv() => {
					let batch = match batch {
						Some(b) => b,
						None => return Ok(()), // This means the connection has to be closed, as the handle was dropped
					};
					for packet in batch {
						match packet {
							PacketS2C::Legacy(response) => break 'outer response,
							_ => continue, // some other packet, we only need LegacyPingResponse
						}
					}
				},
			};
		};

		write_legacy_response(&mut writer.stream, legacy_ping_format, response).await?;
		return Ok(()); // close the connection
	}

	// Ok so its not a legacy ping, lets continue with the normal handshake

	// we will read the handshake in this task before splitting into two tasks
	// so we know the next state for both tasks

	let handshake = match timeout(Duration::from_secs(5), reader.read_packet()).await {
		Ok(p) => p.context("reading packet")?,
		Err(_) => bail!("timed out"),
	};

	let handshake = match handshake {
		PacketC2S::HandshakeC2S(HandshakeC2S::Handshake(handshake)) => handshake,
		_ => unreachable!(),
	};

	// set the client protocol version
	client_protocol_version
		.set(handshake.protocol_version.0)
		.expect("client protocol version already set");

	packet_receiver
		.send(handshake.clone().into_packet())
		.await?;

	// update the state of the reader and writer
	let state = match handshake.next_state {
		NextState::Status => ConnState::Status,
		NextState::Login | NextState::Transfer => ConnState::Login,
	};
	reader.state = state;
	writer.state = state;

	// now we can finally split into two tasks
	// spawn a task to handle writing packets
	// since we now know the state
	spawn(writer_task(writer, packet_sender, packet_batch_sender));

	// continue reading packets in this task
	loop {
		let packet = reader.read_packet().await?;

		// match certain special packets that change the state
		match &packet {
			PacketC2S::LoginC2S(LoginC2S::LoginAcknowledged(_)) => {
				reader.state = ConnState::Configuration;
			}
			_ => {}
		}

		packet_receiver.send(packet).await?;
	}
}

/// The task that handles writing packets to the client.
async fn writer_task(
	mut writer: PacketWriter,
	mut packet_sender: UnboundedReceiver<PacketS2C>,
	mut packet_batch_sender: UnboundedReceiver<Vec<PacketS2C>>,
) -> anyhow::Result<()> {
	loop {
		select! {
			packet = packet_sender.recv() => {
				let packet = match packet {
					Some(p) => p,
					None => return Ok(()), // This means the connection has to be closed, as the handle was dropped
				};
				writer.send(packet).await?;
			},
			batch = packet_batch_sender.recv() => {
				let batch = match batch {
					Some(b) => b,
					None => return Ok(()), // This means the connection has to be closed, as the handle was dropped
				};
				for packet in batch {
					writer.send(packet).await?;
				}
			},
		}
	}
}
