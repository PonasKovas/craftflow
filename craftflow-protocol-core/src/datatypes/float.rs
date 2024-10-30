use crate::{MCPRead, MCPWrite, Result};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::Write;

impl<'a> MCPRead<'a> for f32 {
	fn read(input: &[u8]) -> Result<(&[u8], Self)> {
		let r = input.as_ref().read_f32::<BigEndian>()?;

		Ok((&input[4..], r))
	}
}
impl MCPWrite for f32 {
	fn write(&self, output: &mut impl Write) -> Result<usize> {
		output.write_f32::<BigEndian>(*self)?;
		Ok(4)
	}
}

impl<'a> MCPRead<'a> for f64 {
	fn read(input: &[u8]) -> Result<(&[u8], Self)> {
		let r = input.as_ref().read_f64::<BigEndian>()?;

		Ok((&input[8..], r))
	}
}
impl MCPWrite for f64 {
	fn write(&self, output: &mut impl Write) -> Result<usize> {
		output.write_f64::<BigEndian>(*self)?;
		Ok(8)
	}
}
