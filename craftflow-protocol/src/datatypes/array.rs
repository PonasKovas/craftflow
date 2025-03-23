use super::{MCP, MCPRead, MCPWrite, VarInt};
use crate::{Error, Result};
use std::{any::type_name, fmt::Debug, marker::PhantomData};

/// A generic sequence of elements of type `T`, length prefixed as type `LEN` (in the MCP format).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Array<T, LEN = VarInt> {
	_phantom: PhantomData<fn(T, LEN) -> (T, LEN)>,
}

impl<T: MCP, LEN> MCP for Array<T, LEN> {
	type Data = Vec<T::Data>;
}

impl<'a, T, LEN> MCPRead<'a> for Array<T, LEN>
where
	T: MCPRead<'a>,
	LEN: MCPRead<'a>,
	LEN::Data: TryInto<usize> + Into<i128> + Copy,
{
	fn mcp_read(input: &mut &'a [u8]) -> Result<Self::Data> {
		let len = LEN::mcp_read(input)?;
		let len_i128 = len.into();
		let len: usize = len
			.try_into()
			.map_err(|_| Error::InvalidArrayLength(len_i128))?;

		let mut data = Vec::new();
		for _ in 0..len {
			data.push(T::mcp_read(input)?);
		}

		Ok(data)
	}
}
impl<T, LEN> MCPWrite for Array<T, LEN>
where
	usize: TryInto<LEN::Data>,
	T: MCPWrite,
	LEN: MCPWrite,
{
	fn mcp_write(data: &Self::Data, output: &mut Vec<u8>) -> usize {
		let mut written = 0;

		let len: LEN::Data = data.len().try_into().unwrap_or_else(|_| {
			panic!(
				"Array length {} could not be converted to {}",
				data.len(),
				type_name::<LEN::Data>()
			)
		});

		written += LEN::mcp_write(&len, output);
		for element in data.iter() {
			written += T::mcp_write(element, output);
		}

		written
	}
}
