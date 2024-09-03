use std::io::Read;

use crate::{MCPReadable, MCPWritable};
use byteorder::{ReadBytesExt, WriteBytesExt};

impl MCPReadable for bool {
	fn read(source: &mut impl Read) -> anyhow::Result<bool> {
		Ok(source.read_u8()? != 0)
	}
}

impl MCPWritable for bool {
	fn write(&self, to: &mut impl std::io::Write) -> anyhow::Result<usize> {
		to.write_u8(*self as u8)?;
		Ok(1)
	}
}
