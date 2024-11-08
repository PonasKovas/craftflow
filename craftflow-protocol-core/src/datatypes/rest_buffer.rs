use crate::{MCPRead, MCPWrite, Result};
use shallowclone::ShallowClone;
use std::{
	io::Write,
	ops::{Deref, DerefMut},
};

#[derive(ShallowClone, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[shallowclone(cow)]
pub enum RestBuffer<'a> {
	#[shallowclone(owned)]
	Owned(Vec<u8>),
	#[shallowclone(borrowed)]
	Borrowed(&'a [u8]),
}

impl<'a> RestBuffer<'a> {
	pub fn new() -> Self {
		Self::Owned(Vec::new())
	}
}
impl<'a> From<Vec<u8>> for RestBuffer<'a> {
	fn from(t: Vec<u8>) -> Self {
		Self::Owned(t)
	}
}
impl<'a> From<&'a [u8]> for RestBuffer<'a> {
	fn from(t: &'a [u8]) -> Self {
		Self::Borrowed(t)
	}
}
impl<'a> Deref for RestBuffer<'a> {
	type Target = [u8];

	fn deref(&self) -> &Self::Target {
		match self {
			RestBuffer::Owned(t) => t,
			RestBuffer::Borrowed(t) => t,
		}
	}
}
impl<'a> DerefMut for RestBuffer<'a> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			RestBuffer::Owned(t) => t,
			RestBuffer::Borrowed(t) => {
				*self = RestBuffer::Owned(t.to_vec());
				match self {
					RestBuffer::Owned(t) => t,
					_ => unreachable!(),
				}
			}
		}
	}
}

impl<'a> MCPRead<'a> for RestBuffer<'a> {
	fn read(input: &'a [u8]) -> Result<(&'a [u8], Self)> {
		let len = input.len();
		let (l, r) = input.split_at(len);

		Ok((r, l.into()))
	}
}

impl<'a> MCPWrite for RestBuffer<'a> {
	fn write(&self, output: &mut impl Write) -> Result<usize> {
		output.write_all(&self)?;

		Ok(self.len())
	}
}
