use super::{compression::CompressionGetter, encryption::Decryptor, ConnState};
use aes::cipher::BlockDecryptMut;
use anyhow::bail;
use craftflow_protocol::{
	datatypes::VarInt,
	packets::{handshake::HandshakeC2S, login::LoginC2S, status::StatusC2S, PacketC2S},
	MCPReadable,
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
}

impl PacketReader {
	/// Reads a single packet from the client (Cancel-safe)
	pub(crate) async fn read_packet(&mut self) -> anyhow::Result<PacketC2S> {
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
			let start = self.buffer.len();
			let read = self.read().await?;

			// decrypt if encryption enabled
			self.decryptor.if_enabled(|dec| {
				for i in start..(start + read) {
					dec.decrypt_block_mut(&mut [self.buffer[i]].into());
				}
			});
		};

		// if the packet was compressed wrap the bytes in a zlib decompressor
		let packet = if self.compression.enabled().is_some() && uncompressed_data_length != 0 {
			let mut reader = flate2::read::ZlibDecoder::new(&mut packet_bytes);

			// Parse the packet
			let packet = match self.state {
				ConnState::Handshake => HandshakeC2S::read(&mut reader)?.into(),
				ConnState::Status => StatusC2S::read(&mut reader)?.into(),
				ConnState::Login => LoginC2S::read(&mut reader)?.into(),
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
				ConnState::Handshake => HandshakeC2S::read(&mut packet_bytes)?.into(),
				ConnState::Status => StatusC2S::read(&mut packet_bytes)?.into(),
				ConnState::Login => LoginC2S::read(&mut packet_bytes)?.into(),
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
			match VarInt::read(&mut &self.buffer[pos..]) {
				Ok(varint) => break Ok(varint),
				Err(e) => {
					// if its not an IO error that means the data is invalid
					// IO error = not enough bytes need to read more
					// Keep in mind this is reading from an in-memory buffer, not the stream
					if !e.is::<std::io::Error>() {
						return Err(e);
					}

					// Read more bytes
					let start = self.buffer.len();
					let read = self.read().await?;

					// Instantly decrypt the bytes we just read if encryption is enabled
					self.decryptor.if_enabled(|dec| {
						for i in start..(start + read) {
							dec.decrypt_block_mut(&mut [self.buffer[i]].into()); // stupid ass cryptography crate with outdated ass generics
						}
					});
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

		self.buffer.extend_from_slice(&temp[..n]);

		Ok(n)
	}
}
