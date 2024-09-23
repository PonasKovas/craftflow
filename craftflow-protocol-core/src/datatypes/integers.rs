use crate::{MCPBaseRead, MCPBaseWrite, Result};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::Write;

impl MCPBaseRead for u8 {
	fn read(_protocol_version: u32, mut input: &[u8]) -> Result<(&[u8], Self)> {
		let r = input.read_u8()?;

		Ok((input, r))
	}
}

impl MCPBaseWrite for u8 {
	fn write(&self, _protocol_version: u32, output: &mut impl Write) -> Result<usize> {
		output.write_u8(*self)?;
		Ok(1)
	}
}

impl MCPBaseRead for u16 {
	fn read(_protocol_version: u32, mut input: &[u8]) -> Result<(&[u8], Self)> {
		let r = input.read_u16::<BigEndian>()?;

		Ok((input, r))
	}
}

impl MCPBaseWrite for u16 {
	fn write(&self, _protocol_version: u32, output: &mut impl Write) -> Result<usize> {
		output.write_u16::<BigEndian>(*self)?;
		Ok(2)
	}
}

impl MCPBaseRead for u32 {
	fn read(_protocol_version: u32, mut input: &[u8]) -> Result<(&[u8], Self)> {
		let r = input.read_u32::<BigEndian>()?;

		Ok((input, r))
	}
}

impl MCPBaseWrite for u32 {
	fn write(&self, _protocol_version: u32, output: &mut impl Write) -> Result<usize> {
		output.write_u32::<BigEndian>(*self)?;
		Ok(4)
	}
}

impl MCPBaseRead for u64 {
	fn read(_protocol_version: u32, mut input: &[u8]) -> Result<(&[u8], Self)> {
		let r = input.read_u64::<BigEndian>()?;

		Ok((input, r))
	}
}

impl MCPBaseWrite for u64 {
	fn write(&self, _protocol_version: u32, output: &mut impl Write) -> Result<usize> {
		output.write_u64::<BigEndian>(*self)?;
		Ok(8)
	}
}

impl MCPBaseRead for u128 {
	fn read(_protocol_version: u32, mut input: &[u8]) -> Result<(&[u8], Self)> {
		let r = input.read_u128::<BigEndian>()?;

		Ok((input, r))
	}
}

impl MCPBaseWrite for u128 {
	fn write(&self, _protocol_version: u32, output: &mut impl Write) -> Result<usize> {
		output.write_u128::<BigEndian>(*self)?;
		Ok(16)
	}
}

impl MCPBaseRead for i8 {
	fn read(_protocol_version: u32, mut input: &[u8]) -> Result<(&[u8], Self)> {
		let r = input.read_i8()?;

		Ok((input, r))
	}
}

impl MCPBaseWrite for i8 {
	fn write(&self, _protocol_version: u32, output: &mut impl Write) -> Result<usize> {
		output.write_i8(*self)?;
		Ok(1)
	}
}

impl MCPBaseRead for i16 {
	fn read(_protocol_version: u32, mut input: &[u8]) -> Result<(&[u8], Self)> {
		let r = input.read_i16::<BigEndian>()?;

		Ok((input, r))
	}
}

impl MCPBaseWrite for i16 {
	fn write(&self, _protocol_version: u32, output: &mut impl Write) -> Result<usize> {
		output.write_i16::<BigEndian>(*self)?;
		Ok(2)
	}
}

impl MCPBaseRead for i32 {
	fn read(_protocol_version: u32, mut input: &[u8]) -> Result<(&[u8], Self)> {
		let r = input.read_i32::<BigEndian>()?;

		Ok((input, r))
	}
}

impl MCPBaseWrite for i32 {
	fn write(&self, _protocol_version: u32, output: &mut impl Write) -> Result<usize> {
		output.write_i32::<BigEndian>(*self)?;
		Ok(4)
	}
}

impl MCPBaseRead for i64 {
	fn read(_protocol_version: u32, mut input: &[u8]) -> Result<(&[u8], Self)> {
		let r = input.read_i64::<BigEndian>()?;

		Ok((input, r))
	}
}

impl MCPBaseWrite for i64 {
	fn write(&self, _protocol_version: u32, output: &mut impl Write) -> Result<usize> {
		output.write_i64::<BigEndian>(*self)?;
		Ok(8)
	}
}

impl MCPBaseRead for i128 {
	fn read(_protocol_version: u32, mut input: &[u8]) -> Result<(&[u8], Self)> {
		let r = input.read_i128::<BigEndian>()?;

		Ok((input, r))
	}
}

impl MCPBaseWrite for i128 {
	fn write(&self, _protocol_version: u32, output: &mut impl Write) -> Result<usize> {
		output.write_i128::<BigEndian>(*self)?;
		Ok(16)
	}
}
