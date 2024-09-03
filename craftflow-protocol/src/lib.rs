pub mod datatypes;
#[macro_use]
mod macros;
pub mod packets;

use anyhow::Result;
use std::io::{Read, Write};

/// For types that can be read from a byte slice in the Minecraft Protocol format
pub trait MCPReadable {
	fn read(source: &mut impl Read) -> Result<Self>
	where
		Self: Sized;
}

/// For types that can be written into a byte slice in the Minecraft Protocol format
pub trait MCPWritable {
	/// Writes the data and returns the number of bytes written
	fn write(&self, to: &mut impl Write) -> Result<usize>;
}
