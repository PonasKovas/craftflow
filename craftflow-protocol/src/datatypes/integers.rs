use crate::{MCPReadable, MCPWritable};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::Read;

impl MCPReadable for u8 {
	fn read(source: &mut impl Read) -> anyhow::Result<u8> {
		Ok(source.read_u8()?)
	}
}

impl MCPReadable for u16 {
	fn read(source: &mut impl Read) -> anyhow::Result<u16> {
		Ok(source.read_u16::<BigEndian>()?)
	}
}

impl MCPReadable for u32 {
	fn read(source: &mut impl Read) -> anyhow::Result<u32> {
		Ok(source.read_u32::<BigEndian>()?)
	}
}

impl MCPReadable for u64 {
	fn read(source: &mut impl Read) -> anyhow::Result<u64> {
		Ok(source.read_u64::<BigEndian>()?)
	}
}

impl MCPReadable for u128 {
	fn read(source: &mut impl Read) -> anyhow::Result<u128> {
		Ok(source.read_u128::<BigEndian>()?)
	}
}

impl MCPReadable for i8 {
	fn read(source: &mut impl Read) -> anyhow::Result<i8> {
		Ok(source.read_i8()?)
	}
}

impl MCPReadable for i16 {
	fn read(source: &mut impl Read) -> anyhow::Result<i16> {
		Ok(source.read_i16::<BigEndian>()?)
	}
}

impl MCPReadable for i32 {
	fn read(source: &mut impl Read) -> anyhow::Result<i32> {
		Ok(source.read_i32::<BigEndian>()?)
	}
}

impl MCPReadable for i64 {
	fn read(source: &mut impl Read) -> anyhow::Result<i64> {
		Ok(source.read_i64::<BigEndian>()?)
	}
}

impl MCPReadable for i128 {
	fn read(source: &mut impl Read) -> anyhow::Result<i128> {
		Ok(source.read_i128::<BigEndian>()?)
	}
}

impl MCPWritable for u8 {
	fn write(&self, to: &mut impl std::io::Write) -> anyhow::Result<usize> {
		to.write_u8(*self)?;
		Ok(1)
	}
}

impl MCPWritable for u16 {
	fn write(&self, to: &mut impl std::io::Write) -> anyhow::Result<usize> {
		to.write_u16::<BigEndian>(*self)?;
		Ok(2)
	}
}

impl MCPWritable for u32 {
	fn write(&self, to: &mut impl std::io::Write) -> anyhow::Result<usize> {
		to.write_u32::<BigEndian>(*self)?;
		Ok(4)
	}
}

impl MCPWritable for u64 {
	fn write(&self, to: &mut impl std::io::Write) -> anyhow::Result<usize> {
		to.write_u64::<BigEndian>(*self)?;
		Ok(8)
	}
}

impl MCPWritable for u128 {
	fn write(&self, to: &mut impl std::io::Write) -> anyhow::Result<usize> {
		to.write_u128::<BigEndian>(*self)?;
		Ok(16)
	}
}

impl MCPWritable for i8 {
	fn write(&self, to: &mut impl std::io::Write) -> anyhow::Result<usize> {
		to.write_i8(*self)?;
		Ok(1)
	}
}

impl MCPWritable for i16 {
	fn write(&self, to: &mut impl std::io::Write) -> anyhow::Result<usize> {
		to.write_i16::<BigEndian>(*self)?;
		Ok(2)
	}
}

impl MCPWritable for i32 {
	fn write(&self, to: &mut impl std::io::Write) -> anyhow::Result<usize> {
		to.write_i32::<BigEndian>(*self)?;
		Ok(4)
	}
}

impl MCPWritable for i64 {
	fn write(&self, to: &mut impl std::io::Write) -> anyhow::Result<usize> {
		to.write_i64::<BigEndian>(*self)?;
		Ok(8)
	}
}

impl MCPWritable for i128 {
	fn write(&self, to: &mut impl std::io::Write) -> anyhow::Result<usize> {
		to.write_i128::<BigEndian>(*self)?;
		Ok(16)
	}
}
