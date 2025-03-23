use super::{MCP, MCPRead, MCPWrite, VarInt, advance};
use crate::{Error, Result};
use std::{any::type_name, fmt::Debug, marker::PhantomData};

/// A sequence of bytes, length prefixed as type `LEN` (in the MCP format).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Buffer<LEN = VarInt> {
	_phantom: PhantomData<fn(LEN) -> LEN>,
}

impl<LEN: MCP> MCP for Buffer<LEN> {
	type Data = Vec<u8>;
}

impl<'a, LEN> MCPRead<'a> for Buffer<LEN>
where
	LEN: MCPRead<'a>,
	LEN::Data: TryInto<usize> + Into<i128> + Copy,
{
	fn mcp_read(input: &mut &'a [u8]) -> Result<Self::Data> {
		let len = LEN::mcp_read(input)?;
		let len_i128 = len.into();
		let len: usize = len
			.try_into()
			.map_err(|_| Error::InvalidArrayLength(len_i128))?;

		if input.len() < len {
			return Err(Error::NotEnoughData(len - input.len()));
		}

		let data = advance(input, len).to_owned();

		Ok(data)
	}
}
impl<LEN> MCPWrite for Buffer<LEN>
where
	usize: TryInto<LEN::Data>,
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
		output.extend_from_slice(&data);

		written + data.len()
	}
}
