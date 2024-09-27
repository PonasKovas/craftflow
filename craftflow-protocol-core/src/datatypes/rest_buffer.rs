use crate::{MCPRead, MCPWrite, Result};
use std::io::Write;

pub struct RestBuffer<T>(pub Vec<T>);

impl<T: MCPRead> MCPRead for RestBuffer<T> {
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

		Ok((input, Self(result)))
	}
}

impl<T: MCPWrite> MCPWrite for RestBuffer<T> {
	fn write(&self, output: &mut impl Write) -> Result<usize> {
		let mut written = 0;

		for element in &self.0 {
			written += element.write(output)?;
		}

		Ok(written)
	}
}
