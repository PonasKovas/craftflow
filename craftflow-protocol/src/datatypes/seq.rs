//! Sequence types.
//! `Seq` for sequences with length inferred (reading to the end of the input stream)
//! `SeqLen` for sequences with a length prefix
//! `SeqN` for sequences with fixed length
//!

use super::VarInt;
use crate::Error;
use crate::MinecraftProtocol;
use crate::Result;
use std::borrow::Cow;
use std::fmt::Debug;
use std::io::Write;
use std::marker::PhantomData;

/// A sequence with the length inferred (reading to the end of the input stream)
#[derive(Debug, Clone, PartialEq)]
pub struct Seq<'a, T: Clone> {
	pub inner: Cow<'a, [T]>,
}

/// A sequence with a length prefix
#[derive(Debug, Clone, PartialEq)]
pub struct SeqLen<'a, T: Clone, L = VarInt> {
	pub inner: Cow<'a, [T]>,
	_phantom: PhantomData<fn(L)>,
}

impl<'a, T: MinecraftProtocol<'a> + Clone> MinecraftProtocol<'a> for Seq<'a, T> {
	fn read(protocol_version: u32, mut input: &'a [u8]) -> Result<(&'a [u8], Self)> {
		let mut result = Vec::new();

		loop {
			match T::read(protocol_version, input) {
				Ok((i, element)) => {
					input = i;
					result.push(element);

					if input.is_empty() {
						break;
					}
				}
				Err(e) => return Err(e),
			}
		}

		Ok((
			input,
			Self {
				inner: Cow::Owned(result),
			},
		))
	}
	fn write(&self, protocol_version: u32, output: &mut impl Write) -> Result<usize> {
		let mut written = 0;

		for element in self.inner.as_ref() {
			written += element.write(protocol_version, output)?;
		}

		Ok(written)
	}
}

impl<'a, T: MinecraftProtocol<'a> + Clone, L> MinecraftProtocol<'a> for SeqLen<'a, T, L>
where
	L: TryInto<usize> + MinecraftProtocol<'a> + Debug,
	usize: TryInto<L>,
	<L as TryInto<usize>>::Error: Debug,
	<usize as TryInto<L>>::Error: Debug,
{
	fn read(protocol_version: u32, input: &'a [u8]) -> Result<(&'a [u8], Self)> {
		let mut result = Vec::new();

		let (mut input, len) = L::read(protocol_version, input)?;
		let len: usize = match len.try_into() {
			Ok(n) => n,
			Err(e) => {
				return Err(Error::InvalidData(format!(
					"Sequence length could not be interpreted as an usize: {e:?}"
				)));
			}
		};

		for _ in 0..len {
			match T::read(protocol_version, input) {
				Ok((i, element)) => {
					input = i;
					result.push(element);
				}
				Err(e) => return Err(e),
			}
		}

		Ok((
			input,
			Self {
				inner: Cow::Owned(result),
				_phantom: PhantomData,
			},
		))
	}
	fn write(&self, protocol_version: u32, output: &mut impl Write) -> Result<usize> {
		let mut written = 0;

		let len: L = match self.inner.len().try_into() {
			Ok(n) => n,
			Err(e) => {
				return Err(Error::InvalidData(format!(
					"Sequence length ({}) could not be converted to the SeqLen length prefix type: {e:?}",
					self.inner.len()
				)));
			}
		};

		written += len.write(protocol_version, output)?;

		for element in self.inner.as_ref() {
			written += element.write(protocol_version, output)?;
		}

		Ok(written)
	}
}

impl<'a, T: Clone> SeqLen<'a, T> {
	pub fn new(inner: Cow<'a, [T]>) -> Self {
		Self {
			inner,
			_phantom: PhantomData,
		}
	}
}
