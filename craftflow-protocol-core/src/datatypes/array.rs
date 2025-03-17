use crate::{Error, MCPRead, MCPWrite, Result};
use std::{
	fmt::Debug,
	io::Write,
	marker::PhantomData,
	ops::{Deref, DerefMut},
};

/// A generic sequence of elements of type `T`, length prefixed as type `LEN` (in the MCP format)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Array<LEN, T> {
	pub inner: Vec<T>,
	_phantom: PhantomData<LEN>,
}

impl<LEN, T: Default> Array<LEN, T> {
	pub fn new() -> Self {
		Self {
			inner: Vec::new(),
			_phantom: PhantomData,
		}
	}
}

impl<LEN, T> Deref for Array<LEN, T> {
	type Target = Vec<T>;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}
impl<'a, LEN, T> DerefMut for Array<'a, LEN, T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

impl<'a, LEN, T> From<Vec<T>> for Array<'a, LEN, T> {
	fn from(value: Vec<T>) -> Self {
		Self {
			inner: value.into(),
			_phantom: PhantomData,
		}
	}
}
impl<'a, LEN, T> From<&'a Vec<T>> for Array<'a, LEN, T> {
	fn from(value: &'a Vec<T>) -> Self {
		Self {
			inner: CoCowSlice::Borrowed(value),
			_phantom: PhantomData,
		}
	}
}

impl<'a, LEN: MCPRead<'a>, T: MCPRead<'a>> MCPRead<'a> for Array<'a, LEN, T>
where
	LEN: TryInto<usize> + Debug + Copy,
	// copy isnt really required but just makes stuff easier here,
	// and I assume there wont be a type that isnt Copy
{
	fn read(input: &'a [u8]) -> Result<(&'a [u8], Self)> {
		let mut data = Vec::new();

		let (mut input, len) = LEN::read(input)?;
		let len: usize = len
			.try_into()
			.map_err(|_| Error::InvalidData(format!("{len:?} could not be converted to usize")))?;

		for _ in 0..len {
			match T::read(input) {
				Ok((i, element)) => {
					input = i;
					data.push(element);
				}
				Err(e) => return Err(e),
			}
		}

		Ok((input, data.into()))
	}
}

impl<'a, LEN: MCPWrite, T: MCPWrite> MCPWrite for Array<'a, LEN, T>
where
	usize: TryInto<LEN>,
{
	fn write(&self, output: &mut impl Write) -> Result<usize> {
		let mut written = 0;

		let len: LEN = self.len().try_into().map_err(|_| {
			Error::InvalidData(format!(
				"Could not convert {} to {}",
				self.len(),
				std::any::type_name::<LEN>()
			))
		})?;

		written += len.write(output)?;

		for element in self.iter() {
			written += element.write(output)?;
		}

		Ok(written)
	}
}
