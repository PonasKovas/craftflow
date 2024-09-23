use super::VarInt;
use crate::{Error, MCPBaseRead, MCPBaseWrite, Result};
use core::str;
use std::io::Write;

impl MCPBaseRead for String {
	fn read(protocol_version: u32, input: &[u8]) -> Result<(&[u8], Self)> {
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

		Ok((input, s.to_owned()))
	}
}

impl MCPBaseWrite for String {
	fn write(&self, protocol_version: u32, output: &mut impl Write) -> Result<usize> {
		let prefix_len = VarInt(self.len() as i32).write(protocol_version, output)?;
		output.write_all(self.as_bytes())?;

		Ok(prefix_len + self.len())
	}
}
