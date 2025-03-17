//! Prefixes the inner type with a boolean, indicating whether the value is present or not.

use crate::Result;
use crate::{MCPRead, MCPWrite};

impl<'a, T: MCPRead<'a>> MCPRead<'a> for Option<T> {
	fn mcp_read(input: &mut &'a [u8]) -> Result<Self> {
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
	fn mcp_write(&self, output: &mut Vec<u8>) -> usize {
		let mut written = 0;

		match self {
			Some(value) => {
				written += true.mcp_write(output);
				written += value.mcp_write(output);
			}
			None => {
				written += false.mcp_write(output);
			}
		}

		written
	}
}
