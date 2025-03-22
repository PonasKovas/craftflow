use super::{MCP, MCPRead, MCPWrite, VarInt, advance};
use crate::{Error, Result, limits::DEFAULT_ARRAY_LEN_LIMIT};
use maxlen::BVec;
use std::{any::type_name, fmt::Debug, marker::PhantomData};

/// A sequence of bytes, length prefixed as type `LEN` (in the MCP format) and `MAX` maximum bytes allowed.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Buffer<const MAX: usize = DEFAULT_ARRAY_LEN_LIMIT, LEN = VarInt> {
	_phantom: PhantomData<fn(LEN) -> LEN>,
}

impl<LEN: MCP, const MAX: usize> MCP for Buffer<MAX, LEN> {
	type Data = BVec<u8, MAX>;
}

impl<'a, LEN, const MAX: usize> MCPRead<'a> for Buffer<MAX, LEN>
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
		let bvec = BVec::from_vec(data).map_err(|e| Error::ArrayTooLong {
			length: e.length,
			max: e.maximum,
		})?;

		Ok(bvec)
	}
}
impl<LEN, const MAX: usize> MCPWrite for Buffer<MAX, LEN>
where
	usize: TryInto<<LEN as MCP>::Data>,
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
