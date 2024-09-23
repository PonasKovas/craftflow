use super::VarInt;
use crate::{MCPBaseRead, MCPBaseWrite, Result};
use std::io::Write;

impl<T: MCPBaseRead> MCPBaseRead for Vec<T> {
	fn read(protocol_version: u32, input: &[u8]) -> Result<(&[u8], Self)> {
		let mut result = Vec::new();

		let (mut input, len) = VarInt::read(protocol_version, input)?;
		let len = len.0 as usize;

		for _ in 0..len {
			match T::read(protocol_version, input) {
				Ok((i, element)) => {
					input = i;
					result.push(element);
				}
				Err(e) => return Err(e),
			}
		}

		Ok((input, result))
	}
}

impl<T: MCPBaseWrite> MCPBaseWrite for Vec<T> {
	fn write(&self, protocol_version: u32, output: &mut impl Write) -> Result<usize> {
		let mut written = 0;

		written += VarInt(self.len() as i32).write(protocol_version, output)?;

		for element in self {
			written += element.write(protocol_version, output)?;
		}

		Ok(written)
	}
}
