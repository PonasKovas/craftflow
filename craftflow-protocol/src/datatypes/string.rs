use super::VarInt;
use crate::{MCPReadable, MCPWritable};
use anyhow::bail;
use std::io::{Read, Write};

impl MCPReadable for String {
	fn read(source: &mut impl Read) -> anyhow::Result<Self> {
		let len = VarInt::read(source)?.0 as usize;

		if len > 1024 * 1024 {
			bail!("String too long ({} bytes)", len);
		}

		let mut buf = vec![0u8; len];
		source.read_exact(&mut buf[..])?;

		let s = String::from_utf8(buf)?;

		Ok(s)
	}
}

impl MCPWritable for String {
	fn write(&self, to: &mut impl Write) -> anyhow::Result<usize> {
		let prefix_len = VarInt(self.len() as i32).write(to)?;
		to.write_all(self.as_bytes())?;

		Ok(prefix_len + self.len())
	}
}
