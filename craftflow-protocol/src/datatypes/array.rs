use super::{MCP, MCPRead, MCPWrite, VarInt};
use crate::{Error, Result, limits::DEFAULT_ARRAY_LEN_LIMIT};
use maxlen::BVec;
use std::{any::type_name, fmt::Debug, marker::PhantomData};

/// A generic sequence of elements of type `T`, length prefixed as type `LEN` (in the MCP format) and `MAX` maximum elements allowed.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Array<T, const MAX: usize = DEFAULT_ARRAY_LEN_LIMIT, LEN = VarInt> {
	_phantom: PhantomData<fn(T, LEN) -> (T, LEN)>,
}

impl<T: MCP, LEN, const MAX: usize> MCP for Array<T, MAX, LEN> {
	type Data = BVec<T::Data, MAX>;
}

impl<'a, T, LEN, const MAX: usize> MCPRead<'a> for Array<T, MAX, LEN>
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

		let bvec = BVec::from_vec(data).map_err(|e| Error::ArrayTooLong {
			length: e.length,
			max: e.maximum,
		})?;

		Ok(bvec)
	}
}
impl<T, LEN, const MAX: usize, LENDATA> MCPWrite for Array<T, MAX, LEN>
where
	usize: TryInto<LENDATA>,
	T: MCPWrite,
	LEN: MCPWrite<Data = LENDATA>,
{
	fn mcp_write(data: &Self::Data, output: &mut Vec<u8>) -> usize {
		let mut written = 0;

		let len: LENDATA = data.len().try_into().unwrap_or_else(|_| {
			panic!(
				"Array length {} could not be converted to {}",
				data.len(),
				type_name::<LENDATA>()
			)
		});

		written += LEN::mcp_write(&len, output);
		for element in data.iter() {
			written += T::mcp_write(element, output);
		}

		written
	}
}
