use super::VarInt;
use crate::{Error, MinecraftProtocol, Result};
use core::str;
use std::borrow::Cow;
use std::io::Write;

impl<'a> MinecraftProtocol<'a> for Cow<'a, str> {
	fn read(protocol_version: u32, input: &'a [u8]) -> Result<(&'a [u8], Self)> {
		let (mut input, len) = VarInt::read(protocol_version, input)?;
		let len = len.0 as usize;

		if len > 1024 * 1024 {
			return Err(Error::InvalidData(format!("String too long {len}")));
		}

		if input.len() < len {
			return Err(Error::InvalidData(format!(
				"string too long to fit in packet"
			)));
		}

		let s = match str::from_utf8(&input[..len]) {
			Ok(s) => s,
			Err(e) => {
				return Err(Error::InvalidData(format!("string not valid UTF-8: {e}")));
			}
		};

		input = &input[len..];

		Ok((input, Cow::Borrowed(s)))
	}
	fn write(&self, protocol_version: u32, output: &mut impl Write) -> Result<usize> {
		let prefix_len = VarInt(self.len() as i32).write(protocol_version, output)?;
		output.write_all(self.as_bytes())?;

		Ok(prefix_len + self.len())
	}
}
