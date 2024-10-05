use crate::{MCPRead, MCPWrite, Result};
use std::io::{self, Write};

/// Serializes elements sequentially, setting the first bit of the last element to 1 to indicate that
/// its the last element.
pub struct TopBitSetArray<T>(pub Vec<T>);

/// Wraps a `Write` and always writes the first bit as 1.
struct LastWriteWrapper<W> {
	inner: W,
	/// Starts as true, set to false after first byte is written
	first_byte: bool,
}

impl<W: Write> LastWriteWrapper<W> {
	pub fn new(inner: W) -> Self {
		Self {
			inner,
			first_byte: true,
		}
	}
}

impl<W: Write> Write for LastWriteWrapper<W> {
	fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
		if buf.is_empty() {
			return Ok(0);
		}

		if self.first_byte {
			self.inner.write_all(&[buf[0] | 0b1000_0000])?;
			self.first_byte = false;

			Ok(self.inner.write(&buf[1..])? + 1)
		} else {
			self.inner.write(buf)
		}
	}
	fn flush(&mut self) -> std::io::Result<()> {
		self.inner.flush()
	}
}

impl<T: MCPWrite> MCPWrite for TopBitSetArray<T> {
	fn write(&self, mut output: &mut impl Write) -> Result<usize> {
		let mut written_bytes = 0;

		let last_element_i = self.0.len() - 1;
		for (i, item) in self.0.iter().enumerate() {
			if i == last_element_i {
				let mut last_write_wrapper = LastWriteWrapper::new(&mut output);
				written_bytes += item.write(&mut last_write_wrapper)?;
			} else {
				written_bytes += item.write(output)?;
			}
		}

		Ok(written_bytes)
	}
}

impl<T: MCPRead> MCPRead for TopBitSetArray<T> {
	fn read(mut input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let mut items = Vec::new();

		if input.is_empty() {
			Err(io::Error::new(
				io::ErrorKind::UnexpectedEof,
				"TopBitSetArray must have at least 1 element",
			))?;
		}

		loop {
			let last_element = if (input[0] & 0b1000_0000) != 0 {
				input[0] &= 0b0111_1111;
				true
			} else {
				false
			};

			let (i, item) = T::read(input)?;
			items.push(item);
			input = i;

			if last_element {
				break;
			}
		}

		Ok((input, Self(items)))
	}
}
