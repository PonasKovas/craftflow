use crate::{MCPBaseRead, MCPBaseWrite, Result};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::Write;

impl MCPBaseRead for f32 {
	fn read(_protocol_version: u32, mut input: &[u8]) -> Result<(&[u8], Self)> {
		let r = input.read_f32::<BigEndian>()?;

		Ok((input, r))
	}
}
impl MCPBaseWrite for f32 {
	fn write(&self, _protocol_version: u32, output: &mut impl Write) -> Result<usize> {
		output.write_f32::<BigEndian>(*self)?;
		Ok(4)
	}
}

impl MCPBaseRead for f64 {
	fn read(_protocol_version: u32, mut input: &[u8]) -> Result<(&[u8], Self)> {
		let r = input.read_f64::<BigEndian>()?;

		Ok((input, r))
	}
}
impl MCPBaseWrite for f64 {
	fn write(&self, _protocol_version: u32, output: &mut impl Write) -> Result<usize> {
		output.write_f64::<BigEndian>(*self)?;
		Ok(8)
	}
}
