use crate::Result;
use crate::{MCPRead, MCPWrite};
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io::Write;

impl<'a> MCPRead<'a> for bool {
	fn read(input: &[u8]) -> Result<(&[u8], bool)> {
		let b = input.as_ref().read_u8()? != 0;

		Ok((&input[1..], b))
	}
}

impl MCPWrite for bool {
	fn write(&self, output: &mut impl Write) -> Result<usize> {
		output.write_u8(*self as u8)?;
		Ok(1)
	}
}
