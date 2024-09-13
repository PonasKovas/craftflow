#![feature(doc_cfg)]

pub mod datatypes;
pub mod protocol;
pub(crate) mod stable_packets;

use std::io::{self, Write};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
	#[error("IO error: {0}")]
	IOError(#[from] io::Error),
	#[error("Invalid data: {0}")]
	InvalidData(String),
}

pub type Result<T> = std::result::Result<T, Error>;

/// Types that can be (de)serialized in the Minecraft network protocol format
pub trait MinecraftProtocol<'a> {
	/// Reads and parses the data, returning the remaining data if any together with the parsed value.
	/// `protocol_version` is the protocol version of the connected peer.
	fn read(protocol_version: u32, input: &'a [u8]) -> Result<(&'a [u8], Self)>
	where
		Self: Sized;

	/// Writes the data and returns the number of bytes written
	/// `protocol_version` is the protocol version of the connected peer.
	fn write(&self, protocol_version: u32, output: &mut impl Write) -> Result<usize>;
}

/// Types that are packets
pub trait Packet {
	type Direction;
	/// If this packet has a lifetime, this should be Self<'static>, if not - simply Self
	/// This is needed due to rust being a little bitch and not allowing to get the type ID
	/// of non-'static types, and we need the type ID of a packet for the event system.
	type StaticSelf: 'static;

	fn into_packet_enum(self) -> Self::Direction;
}

impl Error {
	pub fn is_io_error(&self) -> bool {
		matches!(self, Error::IOError(_))
	}
	pub fn is_invalid_data(&self) -> bool {
		matches!(self, Error::InvalidData(_))
	}
}
