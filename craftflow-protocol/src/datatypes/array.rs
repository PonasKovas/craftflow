use super::VarInt;
use crate::{Error, MCPRead, MCPWrite, Result};
use maxlen::BVec;
use std::{
	any::type_name,
	fmt::Debug,
	marker::PhantomData,
	ops::{Deref, DerefMut},
};

// all arrays that dont specify an explicit limit will have this length limit. might tweak this later.
const DEFAULT_LIMIT: usize = 1024 * 1024;

/// A generic sequence of elements of type `T`, length prefixed as type `LEN` (in the MCP format) and `MAX` maximum elements allowed.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Array<T, const MAX: usize = DEFAULT_LIMIT, LEN = VarInt> {
	pub inner: BVec<T, MAX>,
	_phantom: PhantomData<LEN>,
}

impl<LEN, T: Default, const MAX: usize> Array<T, MAX, LEN> {
	pub fn new() -> Self {
		Self {
			inner: BVec::new(),
			_phantom: PhantomData,
		}
	}
}

impl<LEN, T, const MAX: usize> Deref for Array<T, MAX, LEN> {
	type Target = BVec<T, MAX>;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}
impl<LEN, T, const MAX: usize> DerefMut for Array<T, MAX, LEN> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

impl<LEN, T, const MAX: usize> From<BVec<T, MAX>> for Array<T, MAX, LEN> {
	fn from(value: BVec<T, MAX>) -> Self {
		Self {
			inner: value,
			_phantom: PhantomData,
		}
	}
}

impl<'a, T, LEN, const MAX: usize> MCPRead<'a> for Array<T, MAX, LEN>
where
	T: MCPRead<'a>,
	LEN: MCPRead<'a> + TryInto<usize> + Into<i64> + Copy,
{
	fn mcp_read(input: &mut &'a [u8]) -> Result<Self> {
		let len = LEN::mcp_read(input)?;
		let len_i64 = len.into();
		let len: usize = len
			.try_into()
			.map_err(|_| Error::InvalidArrayLength(len_i64))?;

		let mut data = Vec::new();
		for _ in 0..len {
			data.push(T::mcp_read(input)?);
		}

		let bvec = BVec::from_vec(data).map_err(|e| Error::ArrayTooLong {
			length: e.length,
			max: e.maximum,
		})?;

		Ok(bvec.into())
	}
}
impl<T, LEN, const MAX: usize> MCPWrite for Array<T, MAX, LEN>
where
	usize: TryInto<LEN>,
	T: MCPWrite,
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
		for element in self.iter() {
			written += element.mcp_write(output);
		}

		written
	}
}
