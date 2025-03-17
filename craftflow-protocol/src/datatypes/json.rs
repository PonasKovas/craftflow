use crate::{Error, MCPRead, MCPWrite, Result};
use serde::{Deserialize, Serialize};
use std::{
	fmt::Debug,
	ops::{Deref, DerefMut},
};

#[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq)]
pub struct Json<T> {
	pub inner: T,
}

impl<T> Deref for Json<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}
impl<T> DerefMut for Json<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

impl<'a, T: Deserialize<'a>> MCPRead<'a> for Json<T> {
	fn mcp_read(input: &mut &'a [u8]) -> Result<Self> {
		let s = MCPRead::mcp_read(input)?;

		let value = serde_json::from_str(s).map_err(|e| Error::from(e))?;

		Ok(Self { inner: value })
	}
}
impl<T: Serialize + Debug> MCPWrite for Json<T> {
	fn mcp_write(&self, output: &mut Vec<u8>) -> usize {
		let s = serde_json::to_string(&self.inner)
			.unwrap_or_else(|e| panic!("Failed serializing JSON: {e}\n{:?}", self.inner));

		s.mcp_write(output)
	}
}
