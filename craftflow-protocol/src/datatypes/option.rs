//! Prefixes the inner type with a boolean, indicating whether the value is present or not.

use super::{MCP, MCPRead, MCPWrite};
use crate::Result;

impl<T: MCP> MCP for Option<T> {
	type Data = Option<T::Data>;
}

impl<'a, T: MCPRead<'a>> MCPRead<'a> for Option<T> {
	fn mcp_read(input: &mut &'a [u8]) -> Result<Self::Data> {
		let present = bool::mcp_read(input)?;

		if present {
			let value = T::mcp_read(input)?;
			Ok(Some(value))
		} else {
			Ok(None)
		}
	}
}

impl<T: MCPWrite> MCPWrite for Option<T> {
	fn mcp_write(data: &Self::Data, output: &mut Vec<u8>) -> usize {
		let mut written = 0;

		match data {
			Some(value) => {
				written += bool::mcp_write(&true, output);
				written += T::mcp_write(value, output);
			}
			None => {
				written += bool::mcp_write(&false, output);
			}
		}

		written
	}
}
