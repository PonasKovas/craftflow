use aes::cipher::{BlockEncryptMut, generic_array::GenericArray};
use anyhow::bail;
use craftflow_protocol::{PacketWrite, S2C};
use flate2::write::ZlibEncoder;
use std::io::Write;
use tokio::{io::AsyncWriteExt, net::tcp::OwnedWriteHalf};

use super::{State, common::varint_num_bytes};

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
			S2C::Play(p) if state == State::Play => {
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
		let uncompressed_len = packet.packet_write(buffer, protocol_version);

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
	*cursor -= varint_num_bytes(value);
	write_varint(value, &mut buffer[*cursor..]);
}

fn write_varint(mut varint: i32, output: &mut [u8]) {
	let mut i = 0;

	loop {
		// Take the 7 lower bits of the value
		let mut temp = (varint & 0b0111_1111) as u8;
		varint = ((varint as u32) >> 7) as i32;

		// If there is more data to write, set the high bit
		if varint != 0 {
			temp |= 0b1000_0000;
		}

		output[i] = temp;
		i += 1;

		// If there is no more data to write, exit the loop
		if varint == 0 {
			break;
		}
	}
}
