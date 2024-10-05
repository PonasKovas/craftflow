use crate::{MCPRead, MCPWrite, Result};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::Write;

impl MCPRead for u8 {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let r = input.as_ref().read_u8()?;

		Ok((&mut input[1..], r))
	}
}

impl MCPWrite for u8 {
	fn write(&self, output: &mut impl Write) -> Result<usize> {
		output.write_u8(*self)?;
		Ok(1)
	}
}

impl MCPRead for u16 {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let r = input.as_ref().read_u16::<BigEndian>()?;

		Ok((&mut input[2..], r))
	}
}

impl MCPWrite for u16 {
	fn write(&self, output: &mut impl Write) -> Result<usize> {
		output.write_u16::<BigEndian>(*self)?;
		Ok(2)
	}
}

impl MCPRead for u32 {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let r = input.as_ref().read_u32::<BigEndian>()?;

		Ok((&mut input[4..], r))
	}
}

impl MCPWrite for u32 {
	fn write(&self, output: &mut impl Write) -> Result<usize> {
		output.write_u32::<BigEndian>(*self)?;
		Ok(4)
	}
}

impl MCPRead for u64 {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let r = input.as_ref().read_u64::<BigEndian>()?;

		Ok((&mut input[8..], r))
	}
}

impl MCPWrite for u64 {
	fn write(&self, output: &mut impl Write) -> Result<usize> {
		output.write_u64::<BigEndian>(*self)?;
		Ok(8)
	}
}

impl MCPRead for u128 {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let r = input.as_ref().read_u128::<BigEndian>()?;

		Ok((&mut input[16..], r))
	}
}

impl MCPWrite for u128 {
	fn write(&self, output: &mut impl Write) -> Result<usize> {
		output.write_u128::<BigEndian>(*self)?;
		Ok(16)
	}
}

impl MCPRead for i8 {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let r = input.as_ref().read_i8()?;

		Ok((&mut input[1..], r))
	}
}

impl MCPWrite for i8 {
	fn write(&self, output: &mut impl Write) -> Result<usize> {
		output.write_i8(*self)?;
		Ok(1)
	}
}

impl MCPRead for i16 {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let r = input.as_ref().read_i16::<BigEndian>()?;

		Ok((&mut input[2..], r))
	}
}

impl MCPWrite for i16 {
	fn write(&self, output: &mut impl Write) -> Result<usize> {
		output.write_i16::<BigEndian>(*self)?;
		Ok(2)
	}
}

impl MCPRead for i32 {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let r = input.as_ref().read_i32::<BigEndian>()?;

		Ok((&mut input[4..], r))
	}
}

impl MCPWrite for i32 {
	fn write(&self, output: &mut impl Write) -> Result<usize> {
		output.write_i32::<BigEndian>(*self)?;
		Ok(4)
	}
}

impl MCPRead for i64 {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let r = input.as_ref().read_i64::<BigEndian>()?;

		Ok((&mut input[8..], r))
	}
}

impl MCPWrite for i64 {
	fn write(&self, output: &mut impl Write) -> Result<usize> {
		output.write_i64::<BigEndian>(*self)?;
		Ok(8)
	}
}

impl MCPRead for i128 {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let r = input.as_ref().read_i128::<BigEndian>()?;

		Ok((&mut input[16..], r))
	}
}

impl MCPWrite for i128 {
	fn write(&self, output: &mut impl Write) -> Result<usize> {
		output.write_i128::<BigEndian>(*self)?;
		Ok(16)
	}
}
