use super::ConnState;
use aes::cipher::{generic_array::GenericArray, BlockEncryptMut, KeyIvInit};
use anyhow::bail;
use craftflow_protocol_core::{datatypes::VarInt, MCPWrite};
use craftflow_protocol_versions::{PacketWrite, S2C};
use flate2::write::ZlibEncoder;
use std::{
	io::Cursor,
	sync::{Arc, OnceLock, RwLock},
};
use tokio::{io::AsyncWriteExt, net::tcp::OwnedWriteHalf};

/// Keeps track of the current state of the connection and allows to write packets easily
pub(crate) struct PacketWriter {
	pub(crate) stream: OwnedWriteHalf,
	pub(crate) buffer: Cursor<Vec<u8>>,
	pub(crate) state: Arc<RwLock<ConnState>>,
	pub(crate) encryption_secret: Arc<OnceLock<[u8; 16]>>,
	pub(crate) encryptor: Option<cfb8::Encryptor<aes::Aes128>>,
	pub(crate) compression: Arc<OnceLock<usize>>,
	pub(crate) protocol_version: Arc<OnceLock<u32>>,
}

impl PacketWriter {
	/// Sends a packet to the client, automatically checking if the packet is valid for the current state
	pub(crate) async fn send(&mut self, packet: &S2C) -> anyhow::Result<()> {
		let state = *self.state.read().unwrap();
		match packet {
			S2C::Status(p) if state == ConnState::Status => {
				self.write_unchecked(p).await?;
			}
			_ => {
				bail!(
					"Attempt to send packet on wrong state.\nState: {:?}\nPacket: {:?}",
					self.state,
					packet
				);
			}
		}

		// // match certain special packets that change the state
		// match packet {
		// 	S2C::Login(s2c::LoginPacket::LoginSuccess { packet: _ }) => {
		// 		self.state = ConnState::Configuration;
		// 	}
		// 	S2C::Configuration(s2c::ConfigurationPacket::FinishConfiguration { packet: _ }) => {
		// 		self.state = ConnState::Play;
		// 	}
		// 	_ => {}
		// }

		Ok(())
	}

	/// Writes anything writable as a packet into the stream
	/// Doesnt check if the packet is valid for the current state
	pub(crate) async fn write_unchecked(
		&mut self,
		packet: &impl PacketWrite,
	) -> anyhow::Result<()> {
		self.buffer.get_mut().clear();

		let protocol_version = self.get_protocol_version();

		// leave some space at the start of the buffer so we can prefix with the lengths
		self.buffer.get_mut().extend([0u8; 10]);
		self.buffer.set_position(10);

		// Write the packet to the buffer (applying compression if enabled)
		let bytes: &mut [u8] = match self.compression() {
			Some(compression_threshold) => {
				// compress the packet
				let mut zlib = ZlibEncoder::new(&mut self.buffer, flate2::Compression::new(6));
				let mut uncompressed_len = packet.write_packet(&mut zlib, protocol_version)?;
				zlib.finish()?;

				// if turns out the packet was not big enough to be compressed
				if uncompressed_len < compression_threshold {
					// The packet was not big enough to be compressed
					// so write again now without compression
					self.buffer.get_mut().drain(10..);
					self.buffer.set_position(10);
					packet.write_packet(&mut self.buffer, protocol_version)?;

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
				let len = packet.write_packet(&mut self.buffer, protocol_version)?;

				// and then prepend the length
				let start_pos = 10 - VarInt(len as i32).len();
				self.buffer.set_position(start_pos as u64);
				VarInt(len as i32).write(&mut self.buffer)?;

				&mut self.buffer.get_mut()[start_pos..]
			}
		};

		// encrypt the packet if encryption is enabled
		let mut encrypt = |enc: &mut cfb8::Encryptor<aes::Aes128>| {
			for i in 0..bytes.len() {
				enc.encrypt_block_mut(GenericArray::from_mut_slice(&mut bytes[i..(i + 1)]));
				// stupid ass cryptography crate with outdated ass generics
				// FUCK GENERIC ARRAY
				// hopefully mr compiler will optimize 🥺
			}
		};
		match &mut self.encryptor {
			Some(enc) => encrypt(enc),
			None => {
				// check if maybe the secret was set
				if let Some(secret) = self.encryption_secret.get() {
					let mut enc = cfb8::Encryptor::<aes::Aes128>::new(secret.into(), secret.into());

					encrypt(&mut enc);

					self.encryptor = Some(enc);
				}
			}
		}

		// write the packet to the stream
		self.stream.write_all(bytes).await?;

		Ok(())
	}
	fn compression(&self) -> Option<usize> {
		self.compression.get().map(|t| *t)
	}
	fn get_protocol_version(&self) -> u32 {
		*self
			.protocol_version
			.get()
			.expect("protocol version should be set by the time we try to send packets")
	}
	fn if_encryption_enabled(&mut self, f: impl FnOnce(&mut cfb8::Encryptor<aes::Aes128>)) {}
}
