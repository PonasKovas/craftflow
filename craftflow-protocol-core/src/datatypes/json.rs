use crate::{Error, MCPRead, MCPWrite, Result};
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq)]
pub struct Json<T> {
	pub inner: T,
}

impl<T: for<'de> Deserialize<'de>> MCPRead for Json<T> {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input, s) = String::read(input)?;

		let value = serde_json::from_str(&s).map_err(|e| Error::InvalidData(format!("{e}")))?;

		Ok((input, Self { inner: value }))
	}
}
impl<T: Serialize> MCPWrite for Json<T> {
	fn write(&self, output: &mut impl Write) -> Result<usize> {
		let s =
			serde_json::to_string(&self.inner).map_err(|e| Error::InvalidData(format!("{e}")))?;

		s.write(output)
	}
}
