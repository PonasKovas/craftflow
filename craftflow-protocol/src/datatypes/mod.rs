//! Common datatypes found all throughout the network protocol.
//!

use crate::{Error, Result};

mod array;
mod buffer;
mod nbt;
mod option;
mod position;
mod primitives;
mod rest_buffer;
mod string;
mod varint;
mod varlong;

pub use array::Array;
pub use buffer::Buffer;
pub use nbt::{NamedNbt, Nbt};
pub use position::{PositionV5, PositionV477};
pub use rest_buffer::RestBuffer;
pub use varint::{OptVarInt, VarInt};
pub use varlong::VarLong;

/// Marks a Minecraft Protocol datatype format
#[allow(clippy::upper_case_acronyms)]
pub trait MCP {
	/// The actual data
	type Data;
}

/// Defines how to write the MCP data
pub trait MCPWrite: MCP {
	fn mcp_write(data: &Self::Data, output: &mut Vec<u8>) -> usize;
}

/// Defines how to read the MCP data
pub trait MCPRead<'a>: MCP + Sized {
	fn mcp_read(input: &mut &'a [u8]) -> Result<Self::Data>;
}

// Helper functions for implementations:
////////////////////////////////////////

/// 😊
fn advance<'a>(s: &mut &'a [u8], n: usize) -> &'a [u8] {
	let (l, r) = std::mem::take(s).split_at(n);
	*s = r;
	l
}

/// 👹
fn peek(input: &&[u8]) -> Result<u8> {
	if input.is_empty() {
		return Err(Error::NotEnoughData(1));
	}

	Ok(input[0])
}
