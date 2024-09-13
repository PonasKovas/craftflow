use crate::MinecraftProtocol;
use crate::Result;
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io::Write;

impl<'a> MinecraftProtocol<'a> for bool {
	fn read(_protocol_version: u32, mut input: &'a [u8]) -> Result<(&'a [u8], bool)> {
		let b = input.read_u8()? != 0;

		Ok((input, b))
	}
	fn write(&self, _protocol_version: u32, output: &mut impl Write) -> Result<usize> {
		output.write_u8(*self as u8)?;
		Ok(1)
	}
}
