use crate::{MCPBaseRead, MCPBaseWrite, Result};
use std::io::Write;

impl<T: MCPBaseRead> MCPBaseRead for Box<[T]> {
	fn read(protocol_version: u32, mut input: &[u8]) -> Result<(&[u8], Self)> {
		let mut result = Vec::new();

		loop {
			match T::read(protocol_version, input) {
				Ok((i, element)) => {
					input = i;
					result.push(element);

					if input.is_empty() {
						break;
					}
				}
				Err(e) => return Err(e),
			}
		}

		Ok((input, result.into_boxed_slice()))
	}
}

impl<T: MCPBaseWrite> MCPBaseWrite for Box<[T]> {
	fn write(&self, protocol_version: u32, output: &mut impl Write) -> Result<usize> {
		let mut written = 0;

		for element in self.as_ref() {
			written += element.write(protocol_version, output)?;
		}

		Ok(written)
	}
}
