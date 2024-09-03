//! The implementation for Vec<T> just adds a VarInt length prefix and then
//! writes each element sequentially

use super::VarInt;
use crate::{MCPReadable, MCPWritable};
use anyhow::bail;
use std::io::{Read, Write};

impl<T: MCPReadable> MCPReadable for Vec<T> {
	fn read(source: &mut impl Read) -> anyhow::Result<Self> {
		let len = VarInt::read(source)?.0 as usize;

		if len > 1024 * 1024 {
			bail!("sequence too long ({})", len);
		}

		let mut result = Vec::with_capacity(len);
		for _ in 0..len {
			result.push(T::read(source)?);
		}

		Ok(result)
	}
}

impl<T: MCPWritable> MCPWritable for Vec<T> {
	fn write(&self, to: &mut impl Write) -> anyhow::Result<usize> {
		let mut written = VarInt(self.len() as i32).write(to)?;

		for element in self {
			written += element.write(to)?;
		}

		Ok(written)
	}
}
