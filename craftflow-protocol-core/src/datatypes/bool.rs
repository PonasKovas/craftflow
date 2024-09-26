use crate::Result;
use crate::{MCPRead, MCPWrite};
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io::Write;

impl MCPRead for bool {
	fn read(mut input: &[u8]) -> Result<(&[u8], bool)> {
		let b = input.read_u8()? != 0;

		Ok((input, b))
	}
}

impl MCPWrite for bool {
	fn write(&self, output: &mut impl Write) -> Result<usize> {
		output.write_u8(*self as u8)?;
		Ok(1)
	}
}
