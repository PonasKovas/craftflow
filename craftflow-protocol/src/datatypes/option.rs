//! Prefixes the inner type with a boolean, indicating whether the value is present or not.

use crate::MinecraftProtocol;
use crate::Result;
use std::io::Write;

impl<'a, T: MinecraftProtocol<'a>> MinecraftProtocol<'a> for Option<T> {
	fn read(protocol_version: u32, input: &'a [u8]) -> Result<(&'a [u8], Self)> {
		let (input, tag) = bool::read(protocol_version, input)?;

		if tag {
			let (input, value) = T::read(protocol_version, input)?;
			Ok((input, Some(value)))
		} else {
			Ok((input, None))
		}
	}
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
