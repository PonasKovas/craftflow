use crate::{MCPRead, MCPWrite, Result};
use std::{borrow::Cow, io::Write};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct RestBuffer<'a>(pub Cow<'a, [u8]>);

impl<'a> MCPRead<'a> for RestBuffer<'a> {
	fn read(input: &'a [u8]) -> Result<(&'a [u8], Self)> {
		let len = input.len();
		let (l, r) = input.split_at(len);
		let result = Self(Cow::Borrowed(l));

		Ok((r, result))
	}
}

impl<'a> MCPWrite for RestBuffer<'a> {
	fn write(&self, output: &mut impl Write) -> Result<usize> {
		output.write_all(&self.0)?;

		Ok(self.0.len())
	}
}
