pub mod common_structures;
pub mod datatypes;

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
	fn write(&self, output: &mut impl Write) -> Result<usize>;
}

/// Types that can be deserialized in the Minecraft network protocol format
pub trait MCPRead {
	/// Reads and parses the data, returning the remaining data (if any) together with the parsed value.
	fn read(input: &[u8]) -> Result<(&[u8], Self)>
	where
		Self: Sized;
}

impl Error {
	pub fn is_io_error(&self) -> bool {
		matches!(self, Error::IOError(_))
	}
	pub fn is_invalid_data(&self) -> bool {
		matches!(self, Error::InvalidData(_))
	}
}

pub trait Context {
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
