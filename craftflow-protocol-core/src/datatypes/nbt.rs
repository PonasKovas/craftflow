use crate::{Error, MCPRead, MCPWrite, Result};
use craftflow_nbt::DynNBT;
use serde::{Deserialize, Serialize};
use shallowclone::ShallowClone;
use std::io::Write;

#[derive(ShallowClone, Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq)]
pub struct Nbt<T = DynNBT> {
	pub inner: T,
}

#[derive(ShallowClone, Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq)]
pub struct AnonymousNbt<T = DynNBT> {
	pub inner: T,
}

impl<'a, T: Deserialize<'a>> MCPRead<'a> for Nbt<T> {
	fn read(input: &'a [u8]) -> Result<(&'a [u8], Self)> {
		let (input, value): (_, T) =
			craftflow_nbt::from_slice(input).map_err(|e| Error::InvalidData(e.to_string()))?;

		Ok((input, Self { inner: value }))
	}
}
impl<T: Serialize> MCPWrite for Nbt<T> {
	fn write(&self, output: &mut impl Write) -> Result<usize> {
		Ok(craftflow_nbt::to_writer(output, &self.inner)
			.map_err(|e| Error::InvalidData(e.to_string()))?)
	}
}

impl<'a, T: Deserialize<'a>> MCPRead<'a> for AnonymousNbt<T> {
	fn read(input: &'a [u8]) -> Result<(&'a [u8], Self)> {
		let (input, (name, value)): (_, (_, T)) = craftflow_nbt::from_slice_named(input)
			.map_err(|e| Error::InvalidData(e.to_string()))?;
		assert_eq!(name.as_ref(), "");

		Ok((input, Self { inner: value }))
	}
}
impl<T: Serialize> MCPWrite for AnonymousNbt<T> {
	fn write(&self, output: &mut impl Write) -> Result<usize> {
		Ok(craftflow_nbt::to_writer_named(output, &"", &self.inner)
			.map_err(|e| Error::InvalidData(e.to_string()))?)
	}
}
