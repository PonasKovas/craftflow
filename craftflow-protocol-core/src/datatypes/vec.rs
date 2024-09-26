use super::VarInt;
use crate::{MCPRead, MCPWrite, Result};
use std::io::Write;

impl<T: MCPRead> MCPRead for Vec<T> {
	fn read(input: &[u8]) -> Result<(&[u8], Self)> {
		let mut result = Vec::new();

		let (mut input, len) = VarInt::read(input)?;
		let len = len.0 as usize;

		for _ in 0..len {
			match T::read(input) {
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

impl<T: MCPWrite> MCPWrite for Vec<T> {
	fn write(&self, output: &mut impl Write) -> Result<usize> {
		let mut written = 0;

		written += VarInt(self.len() as i32).write(output)?;

		for element in self {
			written += element.write(output)?;
		}

		Ok(written)
	}
}
