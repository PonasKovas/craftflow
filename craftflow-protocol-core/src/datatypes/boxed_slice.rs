use crate::{MCPRead, MCPWrite, Result};
use std::io::Write;

impl<T: MCPRead> MCPRead for Box<[T]> {
	fn read(mut input: &[u8]) -> Result<(&[u8], Self)> {
		let mut result = Vec::new();

		loop {
			match T::read(input) {
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

impl<T: MCPWrite> MCPWrite for Box<[T]> {
	fn write(&self, output: &mut impl Write) -> Result<usize> {
		let mut written = 0;

		for element in self.as_ref() {
			written += element.write(output)?;
		}

		Ok(written)
	}
}
