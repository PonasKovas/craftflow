use crate::{Error, Result};
use craftflow_nbt::{NbtRead, NbtStr, NbtValue, NbtWrite};
use std::marker::PhantomData;

use super::{MCP, MCPRead, MCPWrite, peek};

#[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq)]
pub struct Nbt<T = NbtValue> {
	phantom: PhantomData<fn(T) -> T>,
}

#[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq)]
pub struct OptNbt<T = NbtValue> {
	phantom: PhantomData<fn(T) -> T>,
}

#[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq)]
pub struct NamedNbt<T = NbtValue> {
	phantom: PhantomData<fn(T) -> T>,
}

#[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq)]
pub struct OptNamedNbt<T = NbtValue> {
	phantom: PhantomData<fn(T) -> T>,
}

impl<T> MCP for Nbt<T> {
	type Data = T;
}
impl<T> MCP for OptNbt<T> {
	type Data = Option<T>;
}
impl<T> MCP for NamedNbt<T> {
	type Data = T;
}
impl<T> MCP for OptNamedNbt<T> {
	type Data = Option<T>;
}

impl<'a, T: NbtRead> MCPRead<'a> for Nbt<T> {
	fn mcp_read(input: &mut &'a [u8]) -> Result<Self::Data> {
		let value = T::nbt_read(input).map_err(Error::from)?;

		Ok(value)
	}
}
impl<'a, T: NbtRead> MCPRead<'a> for OptNbt<T> {
	fn mcp_read(input: &mut &'a [u8]) -> Result<Self::Data> {
		let v = match peek(input)? {
			0 => None,
			_ => Some(T::nbt_read(input).map_err(Error::from)?),
		};

		Ok(v)
	}
}
impl<'a, T: NbtRead> MCPRead<'a> for NamedNbt<T> {
	fn mcp_read(input: &mut &'a [u8]) -> Result<Self::Data> {
		let (_, value) = T::nbt_read_named(input).map_err(Error::from)?;

		Ok(value)
	}
}
impl<'a, T: NbtRead> MCPRead<'a> for OptNamedNbt<T> {
	fn mcp_read(input: &mut &'a [u8]) -> Result<Self::Data> {
		let v = match peek(input)? {
			0 => None,
			_ => Some(T::nbt_read_named(input).map_err(Error::from)?.1),
		};

		Ok(v)
	}
}
impl<T: NbtWrite> MCPWrite for Nbt<T> {
	fn mcp_write(data: &Self::Data, output: &mut Vec<u8>) -> usize {
		data.nbt_write(output)
	}
}
impl<T: NbtWrite> MCPWrite for OptNbt<T> {
	fn mcp_write(data: &Self::Data, output: &mut Vec<u8>) -> usize {
		match &data {
			Some(v) => v.nbt_write(output),
			None => {
				output.push(0);
				1
			}
		}
	}
}
impl<T: NbtWrite> MCPWrite for NamedNbt<T> {
	fn mcp_write(data: &Self::Data, output: &mut Vec<u8>) -> usize {
		data.nbt_write_named(NbtStr::new(), output)
	}
}
impl<T: NbtWrite> MCPWrite for OptNamedNbt<T> {
	fn mcp_write(data: &Self::Data, output: &mut Vec<u8>) -> usize {
		match &data {
			Some(v) => v.nbt_write_named(NbtStr::new(), output),
			None => {
				output.push(0);
				1
			}
		}
	}
}
