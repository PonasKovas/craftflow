use super::{compression::Compression, encryption::Encryptor, ConnState};
use aes::cipher::{AsyncStreamCipher, BlockEncryptMut, KeyIvInit};
use anyhow::bail;
use craftflow_protocol::{
	datatypes::VarInt,
	packets::{login::LoginS2C, IntoPacketS2C, PacketS2C},
	MCPWritable,
};
use flate2::write::ZlibEncoder;
use std::{
	io::Cursor,
	sync::{Arc, Mutex, OnceLock},
};
use tokio::{io::AsyncWriteExt, net::tcp::OwnedWriteHalf, select, sync::mpsc::UnboundedReceiver};

/// Keeps track of the current state of the connection and allows to write packets easily
pub(crate) struct PacketWriter {
	pub(crate) stream: OwnedWriteHalf,
	pub(crate) buffer: Cursor<Vec<u8>>,
	pub(crate) state: ConnState,
	pub(crate) encryptor: Encryptor,
	pub(crate) compression: Compression,
}

impl PacketWriter {
	/// Sends a packet to the client, automatically checking if the packet is valid for the current state
	pub(crate) async fn send(&mut self, packet: PacketS2C) -> anyhow::Result<()> {
		match packet {
			PacketS2C::StatusS2C(p) if self.state == ConnState::Status => {
				self.write_unchecked(&p).await?;
			}
			PacketS2C::LoginS2C(p) if self.state == ConnState::Login => {
				self.write_unchecked(&p).await?;
			}
			_ => {
				panic!(
					"Attempt to send packet on wrong state.\nState: {:?}\nPacket: {:?}",
					self.state, packet
				);
			}
		}

		Ok(())
	}

	/// Writes anything writable as a packet into the stream
	/// Doesnt check if the packet is valid for the current state
	pub(crate) async fn write_unchecked(
		&mut self,
		packet: &impl MCPWritable,
	) -> anyhow::Result<()> {
		self.buffer.get_mut().clear();

		// leave some space at the start of the buffer so we can prefix with the lengths
		self.buffer.get_mut().extend([0u8; 10]);
		self.buffer.set_position(10);

		// Write the packet to the buffer (applying compression if enabled)
		let bytes: &mut [u8] = match self.compression.enabled() {
			Some(compression_threshold) => {
				// compress the packet
				let mut zlib = ZlibEncoder::new(&mut self.buffer, flate2::Compression::new(6));
				let mut uncompressed_len = packet.write(&mut zlib)?;
				zlib.finish()?;

				// if turns out the packet was not big enough to be compressed
				if uncompressed_len < compression_threshold {
					// The packet was not big enough to be compressed
					// so write again now without compression
					self.buffer.get_mut().drain(10..);
					self.buffer.set_position(10);
					let len = packet.write(&mut self.buffer)?;

					// write 0 for the uncompressed data length to indicate no compression
					uncompressed_len = 0;
				};

				// add the uncompressed length
				let mut start_pos = 10 - VarInt(uncompressed_len as i32).len();
				self.buffer.set_position(start_pos as u64);
				VarInt(uncompressed_len as i32).write(&mut self.buffer)?;

				// add the full length of the packet
				let packet_len = self.buffer.get_ref().len() - start_pos;
				start_pos -= VarInt(packet_len as i32).len();
				self.buffer.set_position(start_pos as u64);
				VarInt(packet_len as i32).write(&mut self.buffer)?;

				&mut self.buffer.get_mut()[start_pos..]
			}
			None => {
				// no compression so just write the packet
				let len = packet.write(&mut self.buffer)?;

				// and then prepend the length
				let start_pos = 10 - VarInt(len as i32).len();
				self.buffer.set_position(start_pos as u64);
				VarInt(len as i32).write(&mut self.buffer)?;

				&mut self.buffer.get_mut()[start_pos..]
			}
		};

		// encrypt the packet if encryption is enabled
		self.encryptor.if_enabled(|enc| {
			for i in 0..bytes.len() {
				enc.encrypt_block_mut(&mut [bytes[i]].into()); // stupid ass cryptography crate with outdated ass generics
			}
		});

		// write the packet to the stream
		self.stream.write_all(bytes).await?;

		Ok(())
	}
}
