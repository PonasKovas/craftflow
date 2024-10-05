use super::VarInt;
use crate::{Error, MCPRead, MCPWrite, Result};
use core::str;
use std::io::Write;

impl MCPRead for String {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (mut input, len) = VarInt::read(input)?;
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
			Ok(s) => s.to_owned(),
			Err(e) => {
				return Err(Error::InvalidData(format!("string not valid UTF-8: {e}")));
			}
		};

		input = &mut input[len..];

		Ok((input, s))
	}
}

impl MCPWrite for String {
	fn write(&self, output: &mut impl Write) -> Result<usize> {
		let prefix_len = VarInt(self.len() as i32).write(output)?;
		output.write_all(self.as_bytes())?;

		Ok(prefix_len + self.len())
	}
}
