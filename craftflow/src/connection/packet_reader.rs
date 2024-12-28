use aes::cipher::{generic_array::GenericArray, BlockDecryptMut};
use anyhow::{bail, Context};
use craftflow_protocol_abstract::State;
use craftflow_protocol_core::{datatypes::VarInt, MCPRead};
use craftflow_protocol_versions::{
	c2s::{Configuration, Handshaking, Login, Status},
	IntoStateEnum, PacketRead, C2S,
};
use flate2::write::ZlibDecoder;
use std::io::Write;
use tokio::{io::AsyncReadExt, net::tcp::OwnedReadHalf};

const MAX_PACKET_SIZE: usize = 2usize.pow(21);
const DEFAULT_BUFFER_SIZE: usize = 4 * 1024;

pub(crate) type Decryptor = cfb8::Decryptor<aes::Aes128>;

/// Specialised BufReader than can read packets in a cancel-safe way
/// and also handles encryption and compression
pub(crate) struct PacketReader {
	pub(crate) stream: OwnedReadHalf,
	pub(crate) buffer: Vec<u8>,
	pub(crate) decompression_buffer: Vec<u8>,
	// If Some, this number of bytes will be removed from the buffer when starting to read a new packet
	last_packet_len: Option<usize>,
}

impl PacketReader {
	pub(crate) fn new(stream: OwnedReadHalf) -> Self {
		Self {
			stream,
			buffer: Vec::with_capacity(DEFAULT_BUFFER_SIZE),
			decompression_buffer: Vec::with_capacity(DEFAULT_BUFFER_SIZE),
			last_packet_len: None,
		}
	}
	/// Reads a single packet from the client (Cancel-safe)
	pub(crate) async fn read_packet<'a>(
		&'a mut self,
		state: State,
		protocol_version: u32,
		compression: Option<usize>,
		decryptor: &mut Option<Decryptor>,
	) -> anyhow::Result<Option<C2S<'a>>> {
		if let Some(last_packet_len) = self.last_packet_len.take() {
			// remove the packet bytes from the buffer
			self.buffer.drain(..last_packet_len);
		}

		// wait for the length of the next packet
		let packet_len = match self.read_varint_at_pos(0, decryptor).await {
			Ok(l) => l,
			Err(e) => {
				// if we get an error while reading the length, it might be that the connection was just closed
				// and in that case we don't want to print any errors, if it was closed cleanly on a packet boundary
				match e {
					craftflow_protocol_core::Error::IOError(ref error) => {
						// make sure there are no unparsed bytes left in the buffer too,
						// which would mean that the conn didnt close on a packet boundary
						if error.kind() == std::io::ErrorKind::UnexpectedEof
							&& self.buffer.is_empty()
						{
							// yep looks like the connection was closed
							// so just return None, signaling that the connection was cleanly closed
							return Ok(None);
						}

						Err(e)
					}
					other => Err(other),
				}
				.context("reading packet length")?
			}
		};

		let mut packet_start = packet_len.len();
		let packet_len = packet_len.0 as usize;

		let total_packet_len = packet_len + packet_start; // the length of the packet including the length prefix

		if packet_len as usize > MAX_PACKET_SIZE {
			bail!("packet len must be less than {MAX_PACKET_SIZE} bytes (got {packet_len} bytes)");
		}

		// if compression is enabled, read the uncompressed data length
		// this will be set to Some(uncompressed_len) if the packet is compressed
		// (threshold was reached)
		let decompressed_len = match compression {
			None => None,
			Some(threshold) => {
				// read the uncompressed data length
				let length = self.read_varint_at_pos(packet_start, decryptor).await?;
				packet_start += length.len();

				let length = length.0;
				if length >= threshold as i32 {
					Some(length as usize)
				} else if length == 0 {
					None
				} else {
					bail!("Invalid decompressed data length: {}", length);
				}
			}
		};

		// now get the actual packet byte slice without the length prefixes
		let mut packet_bytes: &mut [u8] = loop {
			// check if we have enough bytes
			if self.buffer.len() >= total_packet_len {
				break &mut self.buffer[packet_start..total_packet_len];
			}

			// otherwise read more
			self.read(decryptor).await?;
		};

		// if compression enabled
		if let Some(decompressed_len) = decompressed_len {
			// decompress the packet bytes and make sure the length is correct
			self.decompression_buffer.clear();
			let mut writer = ZlibDecoder::new(&mut self.decompression_buffer);
			writer.write_all(packet_bytes)?;
			writer.finish()?;

			if self.decompression_buffer.len() != decompressed_len {
				bail!(
					"Decompressed data length mismatch: expected {}, got {}",
					decompressed_len,
					self.decompression_buffer.len()
				);
			}

			packet_bytes = &mut self.decompression_buffer[..];
		}

		// Parse the packet
		let (remaining, packet) = match state {
			State::Handshake => {
				let (input, packet) = Handshaking::read_packet(packet_bytes, protocol_version)?;
				(input, packet.into_state_enum())
			}
			State::Status => {
				let (input, packet) = Status::read_packet(packet_bytes, protocol_version)?;
				(input, packet.into_state_enum())
			}
			State::Login => {
				let (input, packet) = Login::read_packet(packet_bytes, protocol_version)?;
				(input, packet.into_state_enum())
			}
			State::Configuration => {
				let (input, packet) = Configuration::read_packet(packet_bytes, protocol_version)?;
				(input, packet.into_state_enum())
			}
			State::Play => todo!(),
		};

		// simple sanity test of parsing the packet, all the bytes should have been used to parse it
		if remaining.len() != 0 {
			bail!(
				"Parsed packet and got {} remaining bytes left",
				remaining.len()
			);
		}

		self.last_packet_len = Some(total_packet_len);

		Ok(Some(packet))
	}
	/// Reads a VarInt in a cancel safe way at a specific position in the buffer
	/// without removing the bytes from the buffer
	async fn read_varint_at_pos(
		&mut self,
		pos: usize,
		decryptor: &mut Option<Decryptor>,
	) -> craftflow_protocol_core::Result<VarInt> {
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
					self.read(decryptor).await?;
				}
			}
		}
	}
	/// Reads more data into the buffer
	/// returns how many bytes were read
	async fn read(&mut self, decryptor: &mut Option<Decryptor>) -> std::io::Result<usize> {
		let mut temp = [0u8; 32 * 1024];

		let n = self.stream.read(&mut temp[..]).await?;

		if n == 0 {
			return Err(std::io::ErrorKind::UnexpectedEof.into());
		}

		// Instantly decrypt the bytes we just read if encryption is enabled
		if let Some(decryptor) = decryptor {
			for i in 0..n {
				decryptor.decrypt_block_mut(GenericArray::from_mut_slice(&mut temp[i..(i + 1)]));
			}
		}

		self.buffer.extend_from_slice(&temp[..n]);

		Ok(n)
	}
}
