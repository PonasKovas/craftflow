use crate::MinecraftProtocol;
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io::Read;

impl MinecraftProtocol for bool {
	fn read(_protocol_version: u32, source: &mut impl Read) -> anyhow::Result<bool> {
		Ok(source.read_u8()? != 0)
	}
	fn write(&self, _protocol_version: u32, to: &mut impl std::io::Write) -> anyhow::Result<usize> {
		to.write_u8(*self as u8)?;
		Ok(1)
	}
}
