//! The implementation is similar to `Vec<u8>` but doesn't have the length prefix
//! instead it reads to the end of the buffer

use crate::MinecraftProtocol;
use std::io::{Read, Write};

impl MinecraftProtocol for Box<[u8]> {
	fn read(_protocol_version: u32, source: &mut impl Read) -> anyhow::Result<Self> {
		let mut buf = Vec::new();
		source.read_to_end(&mut buf)?;

		Ok(buf.into_boxed_slice())
	}
	fn write(&self, _protocol_version: u32, to: &mut impl Write) -> anyhow::Result<usize> {
		to.write_all(self)?;

		Ok(self.len())
	}
}
