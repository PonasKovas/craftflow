use crate::{MinecraftProtocol, Result};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::Write;

impl<'a> MinecraftProtocol<'a> for u8 {
	fn read(_protocol_version: u32, mut input: &'a [u8]) -> Result<(&'a [u8], Self)> {
		let r = input.read_u8()?;

		Ok((input, r))
	}
	fn write(&self, _protocol_version: u32, output: &mut impl Write) -> Result<usize> {
		output.write_u8(*self)?;
		Ok(1)
	}
}

impl<'a> MinecraftProtocol<'a> for u16 {
	fn read(_protocol_version: u32, mut input: &'a [u8]) -> Result<(&'a [u8], Self)> {
		let r = input.read_u16::<BigEndian>()?;

		Ok((input, r))
	}
	fn write(&self, _protocol_version: u32, output: &mut impl Write) -> Result<usize> {
		output.write_u16::<BigEndian>(*self)?;
		Ok(2)
	}
}

impl<'a> MinecraftProtocol<'a> for u32 {
	fn read(_protocol_version: u32, mut input: &'a [u8]) -> Result<(&'a [u8], Self)> {
		let r = input.read_u32::<BigEndian>()?;

		Ok((input, r))
	}
	fn write(&self, _protocol_version: u32, output: &mut impl Write) -> Result<usize> {
		output.write_u32::<BigEndian>(*self)?;
		Ok(4)
	}
}

impl<'a> MinecraftProtocol<'a> for u64 {
	fn read(_protocol_version: u32, mut input: &'a [u8]) -> Result<(&'a [u8], Self)> {
		let r = input.read_u64::<BigEndian>()?;

		Ok((input, r))
	}
	fn write(&self, _protocol_version: u32, output: &mut impl Write) -> Result<usize> {
		output.write_u64::<BigEndian>(*self)?;
		Ok(8)
	}
}

impl<'a> MinecraftProtocol<'a> for u128 {
	fn read(_protocol_version: u32, mut input: &'a [u8]) -> Result<(&'a [u8], Self)> {
		let r = input.read_u128::<BigEndian>()?;

		Ok((input, r))
	}
	fn write(&self, _protocol_version: u32, output: &mut impl Write) -> Result<usize> {
		output.write_u128::<BigEndian>(*self)?;
		Ok(16)
	}
}

impl<'a> MinecraftProtocol<'a> for i8 {
	fn read(_protocol_version: u32, mut input: &'a [u8]) -> Result<(&'a [u8], Self)> {
		let r = input.read_i8()?;

		Ok((input, r))
	}
	fn write(&self, _protocol_version: u32, output: &mut impl Write) -> Result<usize> {
		output.write_i8(*self)?;
		Ok(1)
	}
}

impl<'a> MinecraftProtocol<'a> for i16 {
	fn read(_protocol_version: u32, mut input: &'a [u8]) -> Result<(&'a [u8], Self)> {
		let r = input.read_i16::<BigEndian>()?;

		Ok((input, r))
	}
	fn write(&self, _protocol_version: u32, output: &mut impl Write) -> Result<usize> {
		output.write_i16::<BigEndian>(*self)?;
		Ok(2)
	}
}

impl<'a> MinecraftProtocol<'a> for i32 {
	fn read(_protocol_version: u32, mut input: &'a [u8]) -> Result<(&'a [u8], Self)> {
		let r = input.read_i32::<BigEndian>()?;

		Ok((input, r))
	}
	fn write(&self, _protocol_version: u32, output: &mut impl Write) -> Result<usize> {
		output.write_i32::<BigEndian>(*self)?;
		Ok(4)
	}
}

impl<'a> MinecraftProtocol<'a> for i64 {
	fn read(_protocol_version: u32, mut input: &'a [u8]) -> Result<(&'a [u8], Self)> {
		let r = input.read_i64::<BigEndian>()?;

		Ok((input, r))
	}
	fn write(&self, _protocol_version: u32, output: &mut impl Write) -> Result<usize> {
		output.write_i64::<BigEndian>(*self)?;
		Ok(8)
	}
}

impl<'a> MinecraftProtocol<'a> for i128 {
	fn read(_protocol_version: u32, mut input: &'a [u8]) -> Result<(&'a [u8], Self)> {
		let r = input.read_i128::<BigEndian>()?;

		Ok((input, r))
	}
	fn write(&self, _protocol_version: u32, output: &mut impl Write) -> Result<usize> {
		output.write_i128::<BigEndian>(*self)?;
		Ok(16)
	}
}
