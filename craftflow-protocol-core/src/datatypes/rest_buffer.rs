use crate::{MCPRead, MCPWrite, Result};
use shallowclone::{MakeOwned, ShallowClone};
use std::{
	borrow::Cow,
	io::Write,
	ops::{Deref, DerefMut},
};

#[derive(ShallowClone, MakeOwned, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct RestBuffer<'a> {
	pub data: Cow<'a, [u8]>,
}

impl<'a> RestBuffer<'a> {
	pub fn new() -> Self {
		Self {
			data: Cow::Owned(Vec::new()),
		}
	}
}
impl<'a> From<Cow<'a, [u8]>> for RestBuffer<'a> {
	fn from(t: Cow<'a, [u8]>) -> Self {
		Self { data: t }
	}
}
impl<'a> From<Vec<u8>> for RestBuffer<'a> {
	fn from(t: Vec<u8>) -> Self {
		Cow::<'a, [u8]>::Owned(t).into()
	}
}
impl<'a> From<&'a [u8]> for RestBuffer<'a> {
	fn from(t: &'a [u8]) -> Self {
		Cow::Borrowed(t).into()
	}
}
impl<'a> Deref for RestBuffer<'a> {
	type Target = Cow<'a, [u8]>;

	fn deref(&self) -> &Self::Target {
		&self.data
	}
}
impl<'a> DerefMut for RestBuffer<'a> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.data
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
