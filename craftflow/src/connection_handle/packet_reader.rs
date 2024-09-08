use std::sync::{Arc, OnceLock};

use super::{compression::CompressionGetter, encryption::Decryptor, ConnState};
use aes::cipher::{generic_array::GenericArray, BlockDecryptMut};
use anyhow::bail;
use craftflow_protocol::{
	datatypes::VarInt,
	protocol::{c2s::LoginPacket, C2S},
	stable_packets::c2s::{handshake::Handshake, StatusPacket},
	MinecraftProtocol,
};
use tokio::{io::AsyncReadExt, net::tcp::OwnedReadHalf};

/// Specialised BufReader than can read packets in a cancel-safe way
/// and also handles encryption and compression
pub(crate) struct PacketReader {
	pub(crate) stream: OwnedReadHalf,
	pub(crate) buffer: Vec<u8>,
	pub(crate) state: ConnState,
	pub(crate) decryptor: Decryptor,
	pub(crate) compression: CompressionGetter,
	pub(crate) protocol_version: Arc<OnceLock<u32>>,
}

impl PacketReader {
	/// Reads a single packet from the client (Cancel-safe)
	pub(crate) async fn read_packet(&mut self) -> anyhow::Result<C2S> {
		// wait for the length of the next packet
		let packet_len = self.read_varint_at_pos(0).await?;

		let mut packet_start = packet_len.len();
		let packet_len = packet_len.0 as usize;

		let total_packet_len = packet_len + packet_start; // the length of the packet including the length prefix

		assert!(
			packet_len as usize <= 2usize.pow(15),
			"packet len must be less than 32768 bytes"
		);

		let uncompressed_data_length = if self.compression.enabled().is_some() {
			// read the uncompressed data length
			let uncompressed_data_length = self.read_varint_at_pos(packet_start).await?;
			packet_start += uncompressed_data_length.len();

			uncompressed_data_length.0 as usize
		} else {
			// compression disabled so the uncompressed data length is the same as the packet length
			packet_len
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

		// if the packet was compressed wrap the bytes in a zlib decompressor
		let packet = if self.compression.enabled().is_some() && uncompressed_data_length != 0 {
			let mut reader = flate2::read::ZlibDecoder::new(&mut packet_bytes);

			// Parse the packet
			let packet = match self.state {
				ConnState::Handshake => Handshake::read(protocol_version, &mut reader)?.into(),
				ConnState::Status => StatusPacket::read(protocol_version, &mut reader)?.into(),
				ConnState::Login => LoginPacket::read(protocol_version, &mut reader)?.into(),
				ConnState::Configuration => todo!(),
				ConnState::Play => todo!(),
			};

			// make sure uncompressed data length is correct
			if reader.total_out() != uncompressed_data_length as u64 {
				bail!(
					"Uncompressed data length mismatch: expected {}, got {}",
					uncompressed_data_length,
					reader.total_out()
				);
			}

			packet
		} else {
			// Parse the packet
			let packet = match self.state {
				ConnState::Handshake => {
					Handshake::read(protocol_version, &mut packet_bytes)?.into()
				}
				ConnState::Status => {
					StatusPacket::read(protocol_version, &mut packet_bytes)?.into()
				}
				ConnState::Login => LoginPacket::read(protocol_version, &mut packet_bytes)?.into(),
				ConnState::Configuration => todo!(),
				ConnState::Play => todo!(),
			};

			packet
		};

		// remove the bytes from the buffer
		self.buffer.drain(..total_packet_len);

		Ok(packet)
	}
	/// Reads a VarInt in a cancel safe way at a specific position in the buffer
	async fn read_varint_at_pos(&mut self, pos: usize) -> anyhow::Result<VarInt> {
		loop {
			match VarInt::read(self.get_protocol_version(), &mut &self.buffer[pos..]) {
				Ok(varint) => break Ok(varint),
				Err(e) => {
					// if its not an IO error that means the data is invalid
					// IO error = not enough bytes need to read more
					// Keep in mind this is reading from an in-memory buffer, not the stream
					if !e.is::<std::io::Error>() {
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
