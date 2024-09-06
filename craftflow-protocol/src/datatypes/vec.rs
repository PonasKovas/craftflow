//! The implementation for Vec<T> just adds a VarInt length prefix and then
//! writes each element sequentially

use super::VarInt;
use crate::MinecraftProtocol;
use anyhow::bail;
use std::io::{Read, Write};

impl<T: MinecraftProtocol> MinecraftProtocol for Vec<T> {
	fn read(protocol_version: u32, source: &mut impl Read) -> anyhow::Result<Self> {
		let len = VarInt::read(protocol_version, source)?.0 as usize;

		if len > 1024 * 1024 {
			bail!("sequence too long ({})", len);
		}

		let mut result = Vec::with_capacity(len);
		for _ in 0..len {
			result.push(T::read(protocol_version, source)?);
		}

		Ok(result)
	}
	fn write(&self, protocol_version: u32, to: &mut impl Write) -> anyhow::Result<usize> {
		let mut written = VarInt(self.len() as i32).write(protocol_version, to)?;

		for element in self {
			written += element.write(protocol_version, to)?;
		}

		Ok(written)
	}
}
