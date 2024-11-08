use crate::{Error, MCPRead, MCPWrite, Result};
use shallowclone::ShallowClone;
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
	pub inner: ArrayInner<'a, Vec<T>>,
	_phantom: PhantomData<LEN>,
}

/// The reason why we're not just using Cow here:
/// Cow doesn't work well with shallow clone, since it's invariant over T,
/// which introduces problems when T has a lifetime param. If we used Cow we would be forced
/// to have two lifetime parameters basically everywhere for no reason, because
/// when we shallow clone it we shorten the lifetime of the cow, but can't shorten the lifetime
/// of the inner T, and so we end up with two different lifetimes. The following cow implementation
/// is simpler, not relying on ToOwned trait and is covariant over T, therefore not having this problem.
///
/// This implementation adds the limitation of forcing us to use owned data types like Vec<T>
/// and similar, where we don't necessarily need them, where something like &'static [T] would suffice,
/// but instead we need to use &'static Vec<T>, but it hardly matters.
#[derive(ShallowClone, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[shallowclone(cow)]
pub enum ArrayInner<'a, #[shallowclone(skip)] T> {
	#[shallowclone(owned)]
	Owned(T),
	#[shallowclone(borrowed)]
	Borrowed(&'a T),
}

impl<'a, LEN, T: Default> Array<'a, LEN, T> {
	pub fn new() -> Self {
		Self {
			inner: ArrayInner::new(),
			_phantom: PhantomData,
		}
	}
}
impl<'a, T: Default> ArrayInner<'a, T> {
	pub fn new() -> Self {
		Self::Owned(T::default())
	}
}

impl<'a, LEN, T> Deref for Array<'a, LEN, T> {
	type Target = ArrayInner<'a, Vec<T>>;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}
impl<'a, LEN, T> DerefMut for Array<'a, LEN, T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

impl<'a, T> Deref for ArrayInner<'a, T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		match self {
			ArrayInner::Owned(t) => t,
			ArrayInner::Borrowed(t) => t,
		}
	}
}
impl<'a, T: Clone> DerefMut for ArrayInner<'a, T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			ArrayInner::Owned(t) => t,
			ArrayInner::Borrowed(t) => {
				*self = ArrayInner::Owned(t.clone());
				match self {
					ArrayInner::Owned(t) => t,
					_ => unreachable!(),
				}
			}
		}
	}
}

impl<'a, LEN, T> From<Vec<T>> for Array<'a, LEN, T> {
	fn from(value: Vec<T>) -> Self {
		Self {
			inner: ArrayInner::Owned(value),
			_phantom: PhantomData,
		}
	}
}
impl<'a, LEN, T> From<&'a Vec<T>> for Array<'a, LEN, T> {
	fn from(value: &'a Vec<T>) -> Self {
		Self {
			inner: ArrayInner::Borrowed(value),
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
