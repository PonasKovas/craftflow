use crate::MinecraftProtocol;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::Read;

impl MinecraftProtocol for u8 {
	fn read(_protocol_version: u32, source: &mut impl Read) -> anyhow::Result<u8> {
		Ok(source.read_u8()?)
	}
	fn write(&self, _protocol_version: u32, to: &mut impl std::io::Write) -> anyhow::Result<usize> {
		to.write_u8(*self)?;
		Ok(1)
	}
}

impl MinecraftProtocol for u16 {
	fn read(_protocol_version: u32, source: &mut impl Read) -> anyhow::Result<u16> {
		Ok(source.read_u16::<BigEndian>()?)
	}
	fn write(&self, _protocol_version: u32, to: &mut impl std::io::Write) -> anyhow::Result<usize> {
		to.write_u16::<BigEndian>(*self)?;
		Ok(2)
	}
}

impl MinecraftProtocol for u32 {
	fn read(_protocol_version: u32, source: &mut impl Read) -> anyhow::Result<u32> {
		Ok(source.read_u32::<BigEndian>()?)
	}
	fn write(&self, _protocol_version: u32, to: &mut impl std::io::Write) -> anyhow::Result<usize> {
		to.write_u32::<BigEndian>(*self)?;
		Ok(4)
	}
}

impl MinecraftProtocol for u64 {
	fn read(_protocol_version: u32, source: &mut impl Read) -> anyhow::Result<u64> {
		Ok(source.read_u64::<BigEndian>()?)
	}
	fn write(&self, _protocol_version: u32, to: &mut impl std::io::Write) -> anyhow::Result<usize> {
		to.write_u64::<BigEndian>(*self)?;
		Ok(8)
	}
}

impl MinecraftProtocol for u128 {
	fn read(_protocol_version: u32, source: &mut impl Read) -> anyhow::Result<u128> {
		Ok(source.read_u128::<BigEndian>()?)
	}
	fn write(&self, _protocol_version: u32, to: &mut impl std::io::Write) -> anyhow::Result<usize> {
		to.write_u128::<BigEndian>(*self)?;
		Ok(16)
	}
}

impl MinecraftProtocol for i8 {
	fn read(_protocol_version: u32, source: &mut impl Read) -> anyhow::Result<i8> {
		Ok(source.read_i8()?)
	}
	fn write(&self, _protocol_version: u32, to: &mut impl std::io::Write) -> anyhow::Result<usize> {
		to.write_i8(*self)?;
		Ok(1)
	}
}

impl MinecraftProtocol for i16 {
	fn read(_protocol_version: u32, source: &mut impl Read) -> anyhow::Result<i16> {
		Ok(source.read_i16::<BigEndian>()?)
	}
	fn write(&self, _protocol_version: u32, to: &mut impl std::io::Write) -> anyhow::Result<usize> {
		to.write_i16::<BigEndian>(*self)?;
		Ok(2)
	}
}

impl MinecraftProtocol for i32 {
	fn read(_protocol_version: u32, source: &mut impl Read) -> anyhow::Result<i32> {
		Ok(source.read_i32::<BigEndian>()?)
	}
	fn write(&self, _protocol_version: u32, to: &mut impl std::io::Write) -> anyhow::Result<usize> {
		to.write_i32::<BigEndian>(*self)?;
		Ok(4)
	}
}

impl MinecraftProtocol for i64 {
	fn read(_protocol_version: u32, source: &mut impl Read) -> anyhow::Result<i64> {
		Ok(source.read_i64::<BigEndian>()?)
	}
	fn write(&self, _protocol_version: u32, to: &mut impl std::io::Write) -> anyhow::Result<usize> {
		to.write_i64::<BigEndian>(*self)?;
		Ok(8)
	}
}

impl MinecraftProtocol for i128 {
	fn read(_protocol_version: u32, source: &mut impl Read) -> anyhow::Result<i128> {
		Ok(source.read_i128::<BigEndian>()?)
	}
	fn write(&self, _protocol_version: u32, to: &mut impl std::io::Write) -> anyhow::Result<usize> {
		to.write_i128::<BigEndian>(*self)?;
		Ok(16)
	}
}
