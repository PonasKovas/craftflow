use crate::{Error, MCPRead, MCPWrite, Result};
use craftflow_nbt::{NbtRead, NbtValue, NbtWrite};
use maxlen::BStr;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq)]
pub struct Nbt<T = NbtValue> {
	pub inner: T,
}

#[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq)]
pub struct OptNbt<T = NbtValue> {
	pub inner: Option<T>,
}

#[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq)]
pub struct NamedNbt<T = NbtValue> {
	pub inner: T,
}

#[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq)]
pub struct OptNamedNbt<T = NbtValue> {
	pub inner: Option<T>,
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
impl<T> Deref for OptNbt<T> {
	type Target = Option<T>;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}
impl<T> DerefMut for OptNbt<T> {
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
impl<T> Deref for OptNamedNbt<T> {
	type Target = Option<T>;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}
impl<T> DerefMut for OptNamedNbt<T> {
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
impl<'a, T: NbtRead> MCPRead<'a> for OptNbt<T> {
	fn mcp_read(input: &mut &'a [u8]) -> Result<Self> {
		let v = match peek(input)? {
			0 => None,
			_ => Some(T::nbt_read(input).map_err(|e| Error::from(e))?),
		};

		Ok(Self { inner: v })
	}
}
impl<'a, T: NbtRead> MCPRead<'a> for NamedNbt<T> {
	fn mcp_read(input: &mut &'a [u8]) -> Result<Self> {
		let (_, value) = T::nbt_read_named(input).map_err(|e| Error::from(e))?;

		Ok(Self { inner: value })
	}
}
impl<'a, T: NbtRead> MCPRead<'a> for OptNamedNbt<T> {
	fn mcp_read(input: &mut &'a [u8]) -> Result<Self> {
		let v = match peek(input)? {
			0 => None,
			_ => Some(T::nbt_read_named(input).map_err(|e| Error::from(e))?.1),
		};

		Ok(Self { inner: v })
	}
}
impl<T: NbtWrite> MCPWrite for Nbt<T> {
	fn mcp_write(&self, output: &mut Vec<u8>) -> usize {
		self.nbt_write(output)
	}
}
impl<T: NbtWrite> MCPWrite for OptNbt<T> {
	fn mcp_write(&self, output: &mut Vec<u8>) -> usize {
		match &self.inner {
			Some(v) => v.nbt_write(output),
			None => {
				output.push(0);
				1
			}
		}
	}
}
impl<T: NbtWrite> MCPWrite for NamedNbt<T> {
	fn mcp_write(&self, output: &mut Vec<u8>) -> usize {
		self.nbt_write_named(BStr::new(), output)
	}
}
impl<T: NbtWrite> MCPWrite for OptNamedNbt<T> {
	fn mcp_write(&self, output: &mut Vec<u8>) -> usize {
		match &self.inner {
			Some(v) => v.nbt_write_named(BStr::new(), output),
			None => {
				output.push(0);
				1
			}
		}
	}
}

fn peek(input: &&[u8]) -> Result<u8> {
	if input.len() < 1 {
		return Err(Error::NotEnoughData(1));
	}

	Ok(input[0])
}
