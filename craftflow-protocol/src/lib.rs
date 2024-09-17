#![feature(doc_cfg)]

pub mod datatypes;
pub mod legacy;
pub mod protocol;
pub mod serde_types;

use std::io::{self, Write};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
	#[error("IO error: {0}")]
	IOError(#[from] io::Error),
	#[error("Invalid data: {0}")]
	InvalidData(String),
	#[error("{0}:\n{1}")]
	WithContext(String, Box<Error>),
}

pub type Result<T> = std::result::Result<T, Error>;

/// Types that can be serialized in the Minecraft network protocol format
pub trait MCPWrite {
	/// Writes the data and returns the number of bytes written
	/// `protocol_version` is the protocol version of the connected peer.
	fn write(&self, protocol_version: u32, output: &mut impl Write) -> Result<usize>;
}

/// Types that can be deserialized in the Minecraft network protocol format
pub trait MCPRead {
	/// Reads and parses the data, returning the remaining data (if any) together with the parsed value.
	/// `protocol_version` is the protocol version of the connected peer.
	fn read(protocol_version: u32, input: &[u8]) -> Result<(&[u8], Self)>
	where
		Self: Sized;
}

/// Types that are packets
pub trait Packet {
	type Direction;

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

trait Context {
	fn with_context(self, context: impl FnOnce() -> String) -> Self;
}

impl<T> Context for Result<T> {
	fn with_context(self, context: impl FnOnce() -> String) -> Self {
		match self {
			Ok(inner) => Ok(inner),
			Err(e) => Err(Error::WithContext(context(), Box::new(e))),
		}
	}
}
