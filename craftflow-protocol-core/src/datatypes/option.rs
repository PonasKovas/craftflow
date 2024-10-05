//! Prefixes the inner type with a boolean, indicating whether the value is present or not.

use crate::Result;
use crate::{MCPRead, MCPWrite};
use std::io::Write;

impl<T: MCPRead> MCPRead for Option<T> {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input, tag) = bool::read(input)?;

		if tag {
			let (input, value) = T::read(input)?;
			Ok((input, Some(value)))
		} else {
			Ok((input, None))
		}
	}
}

impl<T: MCPWrite> MCPWrite for Option<T> {
	fn write(&self, output: &mut impl Write) -> Result<usize> {
		let mut written = 0;

		match self {
			Some(value) => {
				written += true.write(output)?;
				written += value.write(output)?;
			}
			None => {
				written += false.write(output)?;
			}
		}

		Ok(written)
	}
}
