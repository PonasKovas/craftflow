use crate::{Error, MCPRead, MCPWrite, Result};
use std::io::Write;

#[derive(Debug, Clone, PartialEq)]
pub struct Nbt {
	pub inner: crab_nbt::Nbt,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnonymousNbt {
	pub inner: crab_nbt::Nbt,
}

/// Wraps a Write to count the bytes written, since the Nbt parser doesn't return this info
struct Counter<W> {
	inner: W,
	written_bytes: usize,
}
impl<W: Write> Write for Counter<W> {
	fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
		let written = self.inner.write(buf)?;
		self.written_bytes += written;
		Ok(written)
	}
	fn flush(&mut self) -> std::io::Result<()> {
		self.inner.flush()
	}
}

impl MCPRead for Nbt {
	fn read(orig_input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let mut input: &[u8] = orig_input;
		let inner =
			crab_nbt::Nbt::read(&mut input).map_err(|e| Error::InvalidData(format!("{e}")))?;

		let read_bytes = orig_input.len() - input.len();
		Ok((&mut orig_input[read_bytes..], Self { inner }))
	}
}
impl MCPRead for AnonymousNbt {
	fn read(orig_input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let mut input: &[u8] = orig_input;
		let inner = crab_nbt::Nbt::read_unnamed(&mut input)
			.map_err(|e| Error::InvalidData(format!("{e}")))?;

		let read_bytes = orig_input.len() - input.len();
		Ok((&mut orig_input[read_bytes..], Self { inner }))
	}
}

impl MCPWrite for Nbt {
	fn write(&self, output: &mut impl Write) -> Result<usize> {
		let mut counter = Counter {
			inner: output,
			written_bytes: 0,
		};

		self.inner
			.write_to_writer(&mut counter)
			.map_err(|e| match e {
				crab_nbt::error::Error::Io(e) => Error::IOError(e),
				other => Error::InvalidData(format!("{other}")),
			})?;
		Ok(counter.written_bytes)
	}
}
impl MCPWrite for AnonymousNbt {
	fn write(&self, output: &mut impl Write) -> Result<usize> {
		let mut counter = Counter {
			inner: output,
			written_bytes: 0,
		};

		self.inner
			.write_unnamed_to_writer(&mut counter)
			.map_err(|e| match e {
				crab_nbt::error::Error::Io(e) => Error::IOError(e),
				other => Error::InvalidData(format!("{other}")),
			})?;
		Ok(counter.written_bytes)
	}
}
