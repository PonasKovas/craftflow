use crate::{MCPRead, MCPWrite, Result};
use std::io::Write;

pub struct RestBuffer(pub Vec<u8>);

impl MCPRead for RestBuffer {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let len = input.len();
		let r = Self(input.to_vec());

		Ok((&mut input[len..len], r))
	}
}

impl MCPWrite for RestBuffer {
	fn write(&self, output: &mut impl Write) -> Result<usize> {
		output.write_all(&self.0)?;

		Ok(self.0.len())
	}
}
