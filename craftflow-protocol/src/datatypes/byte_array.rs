//! The implementation is similar to Vec<u8> but doesn't have the length prefix
//! instead it reads to the end of the buffer

use crate::{MCPReadable, MCPWritable};
use std::io::{Read, Write};

impl MCPReadable for Box<[u8]> {
	fn read(source: &mut impl Read) -> anyhow::Result<Self> {
		let mut buf = Vec::new();
		source.read_to_end(&mut buf)?;

		Ok(buf.into_boxed_slice())
	}
}

impl MCPWritable for Box<[u8]> {
	fn write(&self, to: &mut impl Write) -> anyhow::Result<usize> {
		to.write_all(self)?;

		Ok(self.len())
	}
}
