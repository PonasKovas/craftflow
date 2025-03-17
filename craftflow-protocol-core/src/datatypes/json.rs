use crate::{Error, MCPRead, MCPWrite, Result};
use serde::{Deserialize, Serialize};
use shallowclone::{MakeOwned, ShallowClone};
use std::{borrow::Cow, io::Write};

#[derive(ShallowClone, MakeOwned, Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq)]
pub struct Json<T> {
	pub inner: T,
}

impl<'a, T: Deserialize<'a>> MCPRead<'a> for Json<T> {
	fn read(input: &'a [u8]) -> Result<(&'a [u8], Self)> {
		let (input, s) = MCPRead::read(input)?;

		let value = serde_json::from_str(s).map_err(|e| Error::InvalidData(format!("{e}")))?;

		Ok((input, Self { inner: value }))
	}
}
impl<T: Serialize> MCPWrite for Json<T> {
	fn write(&self, output: &mut impl Write) -> Result<usize> {
		let s =
			serde_json::to_string(&self.inner).map_err(|e| Error::InvalidData(format!("{e}")))?;

		Cow::<'_, str>::Owned(s).write(output)
	}
}
