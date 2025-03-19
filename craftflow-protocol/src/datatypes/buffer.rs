use super::{VarInt, advance};
use crate::{Error, MCPRead, MCPWrite, Result};
use maxlen::BVec;
use std::{
	any::type_name,
	fmt::Debug,
	marker::PhantomData,
	ops::{Deref, DerefMut},
};

// all buffers that dont specify an explicit limit will have this length limit. might tweak this later.
const DEFAULT_LIMIT: usize = 1024 * 1024;

/// A sequence of bytes, length prefixed as type `LEN` (in the MCP format) and `MAX` maximum bytes allowed.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Buffer<const MAX: usize = DEFAULT_LIMIT, LEN = VarInt> {
	pub inner: BVec<u8, MAX>,
	_phantom: PhantomData<LEN>,
}

impl<LEN, const MAX: usize> Buffer<MAX, LEN> {
	pub fn new() -> Self {
		Self {
			inner: BVec::new(),
			_phantom: PhantomData,
		}
	}
}

impl<LEN, const MAX: usize> Deref for Buffer<MAX, LEN> {
	type Target = BVec<u8, MAX>;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}
impl<LEN, const MAX: usize> DerefMut for Buffer<MAX, LEN> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

impl<LEN, const MAX: usize> From<BVec<u8, MAX>> for Buffer<MAX, LEN> {
	fn from(value: BVec<u8, MAX>) -> Self {
		Self {
			inner: value,
			_phantom: PhantomData,
		}
	}
}

impl<'a, LEN, const MAX: usize> MCPRead<'a> for Buffer<MAX, LEN>
where
	LEN: MCPRead<'a> + TryInto<usize> + Into<i128> + Copy,
{
	fn mcp_read(input: &mut &'a [u8]) -> Result<Self> {
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

		Ok(bvec.into())
	}
}
impl<LEN, const MAX: usize> MCPWrite for Buffer<MAX, LEN>
where
	usize: TryInto<LEN>,
	LEN: MCPWrite,
{
	fn mcp_write(&self, output: &mut Vec<u8>) -> usize {
		let mut written = 0;

		let len: LEN = self.len().try_into().unwrap_or_else(|_| {
			panic!(
				"Array length {} could not be converted to {}",
				self.len(),
				type_name::<LEN>()
			)
		});

		written += len.mcp_write(output);
		output.extend_from_slice(&self);

		written + self.len()
	}
}
