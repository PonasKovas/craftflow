//! Prefixes the inner type with a boolean, indicating whether the value is present or not.

use crate::Result;
use crate::{MCPBaseRead, MCPBaseWrite};
use std::io::Write;

impl<T: MCPBaseRead> MCPBaseRead for Option<T> {
	fn read(protocol_version: u32, input: &[u8]) -> Result<(&[u8], Self)> {
		let (input, tag) = bool::read(protocol_version, input)?;

		if tag {
			let (input, value) = T::read(protocol_version, input)?;
			Ok((input, Some(value)))
		} else {
			Ok((input, None))
		}
	}
}

impl<T: MCPBaseWrite> MCPBaseWrite for Option<T> {
	fn write(&self, protocol_version: u32, output: &mut impl Write) -> Result<usize> {
		let mut written = 0;

		match self {
			Some(value) => {
				written += true.write(protocol_version, output)?;
				written += value.write(protocol_version, output)?;
			}
			None => {
				written += false.write(protocol_version, output)?;
			}
		}

		Ok(written)
	}
}
