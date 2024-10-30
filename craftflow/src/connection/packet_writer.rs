use aes::cipher::{generic_array::GenericArray, BlockEncryptMut};
use anyhow::bail;
use craftflow_protocol_abstract::State;
use craftflow_protocol_core::{datatypes::VarInt, MCPWrite};
use craftflow_protocol_versions::{PacketWrite, S2C};
use flate2::write::ZlibEncoder;
use std::io::Write;
use tokio::{io::AsyncWriteExt, net::tcp::OwnedWriteHalf};

// 0 is no compression, 9 - take as long as you'd like
const COMPRESSION_LEVEL: u32 = 6;
const DEFAULT_BUFFER_SIZE: usize = 4 * 1024;

pub(crate) type Encryptor = cfb8::Encryptor<aes::Aes128>;

/// Keeps track of the current state of the connection and allows to write packets easily
pub(crate) struct PacketWriter {
	pub(crate) stream: OwnedWriteHalf,
	pub(crate) buffer: Vec<u8>,
	pub(crate) compression_buffer: Vec<u8>,
}

impl PacketWriter {
	pub(crate) fn new(stream: OwnedWriteHalf) -> Self {
		Self {
			stream,
			buffer: Vec::with_capacity(DEFAULT_BUFFER_SIZE),
			compression_buffer: Vec::with_capacity(DEFAULT_BUFFER_SIZE),
		}
	}
	/// Sends a packet to the client, automatically checking if the packet is valid for the current state
	pub(crate) async fn send(
		&mut self,
		state: State,
		protocol_version: u32,
		compression: Option<usize>,
		encryptor: &mut Option<Encryptor>,
		packet: &S2C,
	) -> anyhow::Result<()> {
		match packet {
			S2C::Status(p) if state == State::Status => {
				self.write_unchecked(protocol_version, compression, encryptor, p)
					.await?;
			}
			S2C::Login(p) if state == State::Login => {
				self.write_unchecked(protocol_version, compression, encryptor, p)
					.await?;
			}
			S2C::Configuration(p) if state == State::Configuration => {
				self.write_unchecked(protocol_version, compression, encryptor, p)
					.await?;
			}
			_ => {
				bail!(
					"Attempt to send packet on wrong state.\nState: {:?}\nPacket: {:?}",
					state,
					packet
				);
			}
		}

		Ok(())
	}

	/// Writes anything writable as a packet into the stream
	/// Doesnt check if the packet is valid for the current state
	async fn write_unchecked(
		&mut self,
		protocol_version: u32,
		compression: Option<usize>,
		encryptor: &mut Option<Encryptor>,
		packet: &impl PacketWrite,
	) -> anyhow::Result<()> {
		self.buffer.clear();
		self.compression_buffer.clear();

		let mut buffer = &mut self.buffer;

		// leave space at the start of the buffer for two potential varints
		// (length and uncompressed length)
		const MAX_PREFIX: usize = 5 * 2; // 2 varints
		buffer.extend([0u8; MAX_PREFIX]);
		let mut packet_start = MAX_PREFIX;

		// Write the packet to the buffer
		let uncompressed_len = packet.write_packet(buffer, protocol_version)?;

		// compress the packet if compression is enabled
		'compression: {
			if let Some(threshold) = compression {
				if uncompressed_len < threshold {
					// since compression is enabled but we're not compressing
					// set the uncompressed length to 0
					prepend_to_buffer(buffer, &mut packet_start, 0);

					break 'compression;
				}

				self.compression_buffer.resize(packet_start, 0);
				let mut zlib = ZlibEncoder::new(
					&mut self.compression_buffer,
					flate2::Compression::new(COMPRESSION_LEVEL),
				);
				zlib.write_all(&buffer[packet_start..])?;
				zlib.finish()?;

				buffer = &mut self.compression_buffer;

				// write the uncompressed packet length
				prepend_to_buffer(buffer, &mut packet_start, uncompressed_len as i32);
			}
		}

		// write the total packet length
		let total_packet_len = buffer.len() - packet_start;
		prepend_to_buffer(buffer, &mut packet_start, total_packet_len as i32);

		// encrypt the packet if encryption is enabled
		let bytes = &mut buffer[packet_start..];
		if let Some(encryptor) = encryptor {
			for i in 0..bytes.len() {
				encryptor.encrypt_block_mut(GenericArray::from_mut_slice(&mut bytes[i..(i + 1)]));
			}
		}

		// write the packet to the stream
		self.stream.write_all(bytes).await?;

		Ok(())
	}
}

fn prepend_to_buffer(buffer: &mut Vec<u8>, cursor: &mut usize, value: i32) {
	*cursor -= VarInt(value).len();
	VarInt(value).write(&mut &mut buffer[*cursor..]).unwrap();
}
