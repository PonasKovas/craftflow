use super::VarInt;
use crate::{Error, MCPRead, MCPWrite, Result};
use core::str;
use std::{borrow::Cow, io::Write};

impl<'a> MCPRead<'a> for &'a str {
	fn read(input: &'a [u8]) -> Result<(&'a [u8], Self)> {
		let (input, len) = VarInt::read(input)?;
		let len = len.0 as usize;

		if len > 1024 * 1024 {
			return Err(Error::InvalidData(format!("String too long {len}")));
		}

		if input.len() < len {
			return Err(Error::InvalidData(format!(
				"string too long to fit in packet"
			)));
		}

		let (l, r) = input.split_at(len);

		let s = match str::from_utf8(l) {
			Ok(s) => s,
			Err(e) => {
				return Err(Error::InvalidData(format!("string not valid UTF-8: {e}")));
			}
		};

		Ok((r, s))
	}
}

impl<'a> MCPRead<'a> for Cow<'a, str> {
	fn read(input: &'a [u8]) -> Result<(&'a [u8], Self)> {
		let (input, s) = <&str as MCPRead>::read(input)?;

		Ok((input, Cow::Borrowed(s)))
	}
}

impl<'a> MCPWrite for Cow<'a, str> {
	fn write(&self, output: &mut impl Write) -> Result<usize> {
		let prefix_len = VarInt(self.len() as i32).write(output)?;
		output.write_all(self.as_bytes())?;

		Ok(prefix_len + self.len())
	}
}
