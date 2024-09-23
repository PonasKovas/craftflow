use crate::Result;
use crate::{MCPBaseRead, MCPBaseWrite};
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io::Write;

impl MCPBaseRead for bool {
	fn read(_protocol_version: u32, mut input: &[u8]) -> Result<(&[u8], bool)> {
		let b = input.read_u8()? != 0;

		Ok((input, b))
	}
}

impl MCPBaseWrite for bool {
	fn write(&self, _protocol_version: u32, output: &mut impl Write) -> Result<usize> {
		output.write_u8(*self as u8)?;
		Ok(1)
	}
}
