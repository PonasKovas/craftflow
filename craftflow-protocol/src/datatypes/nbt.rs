use crate::{Error, MCPRead, MCPWrite, Result};
use craftflow_nbt::{NbtRead, NbtValue, NbtWrite};
use maxlen::BStr;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq)]
pub struct Nbt<T = NbtValue> {
	pub inner: T,
}

#[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq)]
pub struct NamedNbt<T = NbtValue> {
	pub inner: T,
}

impl<T> Deref for Nbt<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}
impl<T> DerefMut for Nbt<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}
impl<T> Deref for NamedNbt<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}
impl<T> DerefMut for NamedNbt<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

impl<'a, T: NbtRead> MCPRead<'a> for Nbt<T> {
	fn mcp_read(input: &mut &'a [u8]) -> Result<Self> {
		let value = T::nbt_read(input).map_err(|e| Error::from(e))?;

		Ok(Self { inner: value })
	}
}
impl<'a, T: NbtRead> MCPRead<'a> for NamedNbt<T> {
	fn mcp_read(input: &mut &'a [u8]) -> Result<Self> {
		let (_, value) = T::nbt_read_named(input).map_err(|e| Error::from(e))?;

		Ok(Self { inner: value })
	}
}
impl<T: NbtWrite> MCPWrite for Nbt<T> {
	fn mcp_write(&self, output: &mut Vec<u8>) -> usize {
		self.nbt_write(output)
	}
}
impl<T: NbtWrite> MCPWrite for NamedNbt<T> {
	fn mcp_write(&self, output: &mut Vec<u8>) -> usize {
		self.nbt_write_named(BStr::new(), output)
	}
}
