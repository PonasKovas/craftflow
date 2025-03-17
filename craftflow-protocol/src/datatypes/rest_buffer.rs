use crate::{MCPRead, MCPWrite, Result};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct RestBuffer {
	pub data: Vec<u8>,
}

impl RestBuffer {
	pub fn new() -> Self {
		Self { data: Vec::new() }
	}
	pub fn from_vec(v: Vec<u8>) -> Self {
		Self { data: v }
	}
}
impl From<Vec<u8>> for RestBuffer {
	fn from(t: Vec<u8>) -> Self {
		Self { data: t }
	}
}
impl Deref for RestBuffer {
	type Target = Vec<u8>;

	fn deref(&self) -> &Self::Target {
		&self.data
	}
}
impl DerefMut for RestBuffer {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.data
	}
}

impl<'a> MCPRead<'a> for RestBuffer {
	fn mcp_read(input: &mut &'a [u8]) -> Result<Self> {
		Ok(Self::from(input.to_owned()))
	}
}

impl MCPWrite for RestBuffer {
	fn mcp_write(&self, output: &mut Vec<u8>) -> usize {
		output.extend_from_slice(self);

		self.len()
	}
}
