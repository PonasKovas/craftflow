use super::{compression::CompressionGetter, encryption::Decryptor, ConnState};
use aes::cipher::{generic_array::GenericArray, BlockDecryptMut};
use craftflow_protocol::{
	datatypes::VarInt,
	protocol::{
		c2s::{handshake::Handshake, ConfigurationPacket, LoginPacket, StatusPacket},
		C2S,
	},
	Error, MinecraftProtocol,
};
use std::{
	io::Write,
	sync::{Arc, OnceLock},
};
use tokio::{io::AsyncReadExt, net::tcp::OwnedReadHalf};

/// Specialised BufReader than can read packets in a cancel-safe way
/// and also handles encryption and compression
pub(crate) struct PacketReader {
	pub(crate) stream: OwnedReadHalf,
	pub(crate) buffer: Vec<u8>,
	pub(crate) decompression_buffer: Vec<u8>,
	pub(crate) state: ConnState,
	pub(crate) decryptor: Decryptor,
	pub(crate) compression: CompressionGetter,
	pub(crate) protocol_version: Arc<OnceLock<u32>>,
}

impl PacketReader {
	/// Reads a single packet from the client (Cancel-safe)
	/// WARNING: does NOT remove the packet bytes from the buffer
	/// so consequent reads will return the same packet
	/// call pop_packet() manually to move on to next packet!
	pub(crate) async fn read_packet<'a>(&'a mut self) -> craftflow_protocol::Result<C2S<'a>> {
		// wait for the length of the next packet
		let packet_len = self.read_varint_at_pos(0).await?;

		let mut packet_start = packet_len.len();
		let packet_len = packet_len.0 as usize;

		let total_packet_len = packet_len + packet_start; // the length of the packet including the length prefix

		if packet_len as usize > 2usize.pow(15) {
			return Err(Error::InvalidData(format!(
				"packet len must be less than 32768 bytes (got {packet_len} bytes)"
			)));
		}

		let should_decompress = match self.compression.enabled() {
			None => None,
			Some(threshold) => {
				// read the uncompressed data length
				let length = self.read_varint_at_pos(packet_start).await?;
				packet_start += length.len();

				if length.0 as usize >= threshold {
					Some(length.0 as usize)
				} else if length.0 == 0 {
					None
				} else {
					return Err(Error::InvalidData(format!(
						"Invalid decompressed data length: {}",
						length.0
					)));
				}
			}
		};

		// now get the actual packet byte slice without the length prefixes
		let mut packet_bytes: &[u8] = loop {
			// check if we have enough bytes
			if self.buffer.len() >= total_packet_len {
				break &self.buffer[packet_start..total_packet_len];
			}

			// otherwise read more
			self.read().await?;
		};

		let protocol_version = self.get_protocol_version();

		if let Some(decompressed_length) = should_decompress {
			// decompress the packet bytes and make sure the length is correct
			self.decompression_buffer.clear();
			let mut writer = flate2::write::ZlibDecoder::new(&mut self.decompression_buffer);
			writer.write_all(packet_bytes)?;
			writer.finish()?;

			if self.decompression_buffer.len() != decompressed_length {
				return Err(Error::InvalidData(format!(
					"Decompressed data length mismatch: expected {}, got {}",
					decompressed_length,
					self.decompression_buffer.len()
				)));
			}

			packet_bytes = &self.decompression_buffer[..];
		}

		// Parse the packet
		let (remaining, packet) = match self.state {
			ConnState::Handshake => {
				let (input, packet) = Handshake::read(protocol_version, packet_bytes)?;
				(input, packet.into())
			}
			ConnState::Status => {
				let (input, packet) = StatusPacket::read(protocol_version, packet_bytes)?;
				(input, packet.into())
			}
			ConnState::Login => {
				let (input, packet) = LoginPacket::read(protocol_version, packet_bytes)?;
				(input, packet.into())
			}
			ConnState::Configuration => {
				let (input, packet) = ConfigurationPacket::read(protocol_version, packet_bytes)?;
				(input, packet.into())
			}
			ConnState::Play => todo!(),
		};

		if remaining.len() != 0 {
			return Err(Error::InvalidData(format!(
				"Parsed packet and got {} remaining bytes left",
				remaining.len()
			)));
		}

		Ok(packet)
	}
	/// Removes 1 packet from the buffer
	/// Returns an error if there isn't a full packet there
	pub fn pop_packet(&mut self) -> craftflow_protocol::Result<()> {
		let length = VarInt::read(self.get_protocol_version(), &self.buffer[..])?;

		self.buffer
			.drain(..(length.1.len() + (length.1).0 as usize));

		Ok(())
	}
	/// Reads a VarInt in a cancel safe way at a specific position in the buffer
	/// without removing the bytes from the buffer
	async fn read_varint_at_pos(&mut self, pos: usize) -> craftflow_protocol::Result<VarInt> {
		loop {
			match VarInt::read(self.get_protocol_version(), &self.buffer[pos..]) {
				Ok((_input, varint)) => break Ok(varint),
				Err(e) => {
					// if its not an IO error that means the data is invalid
					// IO error = not enough bytes need to read more
					// Keep in mind this is reading from an in-memory buffer, not the stream
					if !e.is_io_error() {
						return Err(e);
					}

					// Read more bytes
					self.read().await?;
				}
			}
		}
	}
	/// Reads more data into the buffer
	/// returns how many bytes were read
	async fn read(&mut self) -> std::io::Result<usize> {
		let mut temp = [0u8; 32 * 1024];

		let n = self.stream.read(&mut temp[..]).await?;

		if n == 0 {
			return Err(std::io::ErrorKind::UnexpectedEof.into());
		}

		// Instantly decrypt the bytes we just read if encryption is enabled
		self.decryptor.if_enabled(|dec| {
			for i in 0..n {
				dec.decrypt_block_mut(GenericArray::from_mut_slice(&mut temp[i..(i + 1)])); // stupid ass cryptography crate with outdated ass generics
				                                                                // FUCK GENERIC ARRAY
				                                                                // hopefully mr compiler will optimize 🥺
			}
		});

		self.buffer.extend_from_slice(&temp[..n]);

		Ok(n)
	}
	fn get_protocol_version(&self) -> u32 {
		match self.protocol_version.get() {
			Some(v) => *v,
			None => {
				// if protocol version is not set, we are in the handshake state, before receiving the handshake packet
				// so in order to read the first packet (which should really be the same for all versions)
				// we just use whatever version we support
				craftflow_protocol::protocol::SUPPORTED_PROTOCOL_VERSIONS[0]
			}
		}
	}
}
