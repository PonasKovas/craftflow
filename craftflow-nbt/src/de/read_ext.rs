use crate::{Error, Result};
use std::{borrow::Cow, io::Read};

pub trait ByteRead<'a> {
	fn read_u8(&mut self) -> Result<u8>;
	fn read_u16(&mut self) -> Result<u16>;
	fn read_u32(&mut self) -> Result<u32>;
	fn read_u64(&mut self) -> Result<u64>;
	fn read_str(&mut self) -> Result<Cow<'a, str>>;
}

impl<'a> ByteRead<'a> for &'a [u8] {
	fn read_u8(&mut self) -> Result<u8> {
		let mut b = [0];
		self.read_exact(&mut b)?;
		Ok(b[0])
	}
	fn read_u16(&mut self) -> Result<u16> {
		let mut b = [0; 2];
		self.read_exact(&mut b)?;
		Ok(u16::from_be_bytes([b[0], b[1]]))
	}
	fn read_u32(&mut self) -> Result<u32> {
		let mut b = [0; 4];
		self.read_exact(&mut b)?;
		Ok(u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
	}
	fn read_u64(&mut self) -> Result<u64> {
		let mut b = [0; 8];
		self.read_exact(&mut b)?;
		Ok(u64::from_be_bytes([
			b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7],
		]))
	}
	fn read_str(&mut self) -> Result<Cow<'a, str>> {
		let len = self.read_u16()?;
		if len as usize > self.len() {
			return Err(Error::InvalidData(format!(
				"String length {len} exceeds input length {}",
				self.len()
			)));
		}

		let s = &self[..len as usize];
		let s = cesu8::from_java_cesu8(s).map_err(|e| Error::InvalidData(format!("{e}")))?;
		*self = &self[len as usize..];

		Ok(s)
	}
}
