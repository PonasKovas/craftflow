use super::ConnState;
use aes::cipher::{generic_array::GenericArray, BlockDecryptMut, KeyIvInit};
use craftflow_protocol_core::{datatypes::VarInt, Error, MCPRead};
use craftflow_protocol_versions::{
	c2s::{Handshaking, Status},
	IntoStateEnum, PacketRead, C2S, MIN_VERSION,
};
use std::{
	io::Write,
	sync::{Arc, OnceLock, RwLock},
};
use tokio::{io::AsyncReadExt, net::tcp::OwnedReadHalf};

/// Specialised BufReader than can read packets in a cancel-safe way
/// and also handles encryption and compression
pub(crate) struct PacketReader {
	pub(crate) stream: OwnedReadHalf,
	pub(crate) buffer: Vec<u8>,
	pub(crate) decompression_buffer: Vec<u8>,
	pub(crate) state: Arc<RwLock<ConnState>>,
	pub(crate) encryption_secret: Arc<OnceLock<[u8; 16]>>,
	pub(crate) decryptor: Option<cfb8::Decryptor<aes::Aes128>>,
	pub(crate) compression: Arc<OnceLock<usize>>,
	pub(crate) protocol_version: Arc<OnceLock<u32>>,
}

impl PacketReader {
	/// Reads a single packet from the client (Cancel-safe)
	pub(crate) async fn read_packet(&mut self) -> craftflow_protocol_core::Result<C2S> {
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

		let should_decompress = match self.compression() {
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

		let protocol_version = self.get_protocol_version();

		// now get the actual packet byte slice without the length prefixes
		let mut packet_bytes: &mut [u8] = loop {
			// check if we have enough bytes
			if self.buffer.len() >= total_packet_len {
				break &mut self.buffer[packet_start..total_packet_len];
			}

			// otherwise read more
			self.read().await?;
		};

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

			packet_bytes = &mut self.decompression_buffer[..];
		}

		// Parse the packet
		let (remaining, packet) = match *self.state.read().unwrap() {
			ConnState::Handshake => {
				let (input, packet) = Handshaking::read_packet(packet_bytes, protocol_version)?;
				(input, packet.into_state_enum())
			}
			ConnState::Status => {
				let (input, packet) = Status::read_packet(packet_bytes, protocol_version)?;
				(input, packet.into_state_enum())
			}
			_ => todo!(),
		};

		if remaining.len() != 0 {
			return Err(Error::InvalidData(format!(
				"Parsed packet and got {} remaining bytes left",
				remaining.len()
			)));
		}

		// remove the bytes from the buffer
		self.buffer.drain(..total_packet_len);

		Ok(packet)
	}
	/// Reads a VarInt in a cancel safe way at a specific position in the buffer
	/// without removing the bytes from the buffer
	async fn read_varint_at_pos(&mut self, pos: usize) -> craftflow_protocol_core::Result<VarInt> {
		loop {
			match VarInt::read(&mut self.buffer[pos..]) {
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
		self.if_encryption_enabled(|dec| {
			for i in 0..n {
				dec.decrypt_block_mut(GenericArray::from_mut_slice(&mut temp[i..(i + 1)]));
				// stupid ass cryptography crate with outdated ass generics
				// FUCK GENERIC ARRAY
				// hopefully mr compiler will optimize 🥺
			}
		});

		self.buffer.extend_from_slice(&temp[..n]);

		Ok(n)
	}
	fn compression(&self) -> Option<usize> {
		self.compression.get().map(|t| *t)
	}
	pub(crate) fn get_protocol_version(&self) -> u32 {
		match self.protocol_version.get() {
			Some(&v) => {
				// If the state is Status, then still give MIN_VERSION instead of real,
				// because we might not support the real version, but status
				// should be the same (or compatible) for all versions and we still want to respond.
				if *self.state.read().unwrap() == ConnState::Status {
					MIN_VERSION
				} else {
					v
				}
			}
			None => {
				// if protocol version is not set, we are in the handshake state,
				// before receiving the handshake packet
				// so just set to the minimal supported version so we can read the handshake
				MIN_VERSION
			}
		}
	}
	fn if_encryption_enabled(&mut self, f: impl FnOnce(&mut cfb8::Decryptor<aes::Aes128>)) {
		match &mut self.decryptor {
			Some(dec) => f(dec),
			None => {
				// check if maybe the secret was set
				if let Some(secret) = self.encryption_secret.get() {
					let mut dec = cfb8::Decryptor::<aes::Aes128>::new(secret.into(), secret.into());

					f(&mut dec);

					self.decryptor = Some(dec);
				}
			}
		}
	}
}
