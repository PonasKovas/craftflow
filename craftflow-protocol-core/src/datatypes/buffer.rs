use crate::{Error, MCPRead, MCPWrite, Result};
use shallowclone::{MakeOwned, ShallowClone};
use std::fmt::Debug;
use std::io::Write;
use std::{
	borrow::Cow,
	marker::PhantomData,
	ops::{Deref, DerefMut},
};

#[derive(ShallowClone, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Buffer<'a, #[shallowclone(skip)] LEN> {
	pub inner: Cow<'a, [u8]>,
	_phantom: PhantomData<LEN>,
}

// LEN doesnt really need to be Clone, but the derive Clone macro above requires it to be
// and in reality it always will be, so it doesnt matter
impl<'a, LEN: Clone + 'static> MakeOwned for Buffer<'a, LEN> {
	type Owned = Buffer<'static, LEN>;

	fn make_owned(self) -> Self::Owned {
		Self::Owned {
			inner: self.inner.make_owned(),
			_phantom: PhantomData,
		}
	}
}

impl<'a, LEN> Buffer<'a, LEN> {
	pub fn new() -> Self {
		Self {
			inner: Vec::new().into(),
			_phantom: PhantomData,
		}
	}
}

impl<'a, LEN> Default for Buffer<'a, LEN> {
	fn default() -> Self {
		Self::new()
	}
}
impl<'a, LEN> Deref for Buffer<'a, LEN> {
	type Target = Cow<'a, [u8]>;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}
impl<'a, LEN> DerefMut for Buffer<'a, LEN> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

impl<'a, LEN> From<Cow<'a, [u8]>> for Buffer<'a, LEN> {
	fn from(value: Cow<'a, [u8]>) -> Self {
		Self {
			inner: value,
			_phantom: PhantomData,
		}
	}
}
impl<'a, LEN> From<&'a [u8]> for Buffer<'a, LEN> {
	fn from(value: &'a [u8]) -> Self {
		Self::from(Cow::Borrowed(value))
	}
}
impl<'a, LEN> From<Vec<u8>> for Buffer<'a, LEN> {
	fn from(value: Vec<u8>) -> Self {
		Self::from(Cow::Owned(value))
	}
}

impl<'a, LEN: MCPRead<'a>> MCPRead<'a> for Buffer<'a, LEN>
where
	LEN: TryInto<usize> + Debug + Copy,
	// copy isnt really required but just makes stuff easier here,
	// and I assume there wont be a type that isnt Copy
{
	fn read(input: &'a [u8]) -> Result<(&'a [u8], Self)> {
		let (input, len) = LEN::read(input)?;
		let len: usize = len
			.try_into()
			.map_err(|_| Error::InvalidData(format!("{len:?} could not be converted to usize")))?;

		let (data, rem) = input.split_at(len);

		Ok((rem, data.into()))
	}
}

impl<'a, LEN: MCPWrite> MCPWrite for Buffer<'a, LEN>
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
		written += self.len();
		output.write_all(&self)?;

		Ok(written)
	}
}
