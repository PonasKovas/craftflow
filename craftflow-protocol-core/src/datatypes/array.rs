use crate::{Error, MCPRead, MCPWrite, Result};
use shallowclone::{CoCowSlice, MakeOwned, ShallowClone};
use std::{
	fmt::Debug,
	io::Write,
	marker::PhantomData,
	ops::{Deref, DerefMut},
};

/// A generic sequence of elements of type `T`, length prefixed as type `LEN` (in the MCP format),
/// possibly borrowing data with lifetime `'a`.
#[derive(ShallowClone, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Array<'a, #[shallowclone(skip)] LEN, #[shallowclone(skip)] T> {
	pub inner: CoCowSlice<'a, T>,
	_phantom: PhantomData<LEN>,
}

// LEN doesnt really need to be Clone, but the derive Clone macro above requires it to be
// and in reality it always will be, so it doesnt matter
impl<'a, LEN: Clone + 'static, T: MakeOwned> MakeOwned for Array<'a, LEN, T> {
	type Owned = Array<'static, LEN, T::Owned>;

	fn make_owned(self) -> Self::Owned {
		Self::Owned {
			inner: self.inner.make_owned(),
			_phantom: PhantomData,
		}
	}
}

impl<'a, LEN, T: Default> Array<'a, LEN, T> {
	pub fn new() -> Self {
		Self {
			inner: Default::default(),
			_phantom: PhantomData,
		}
	}
}

impl<'a, LEN, T> Deref for Array<'a, LEN, T> {
	type Target = CoCowSlice<'a, T>;

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
