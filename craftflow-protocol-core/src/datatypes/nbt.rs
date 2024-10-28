use crate::{Error, MCPRead, MCPWrite, Result};
use craftflow_nbt::DynNBT;
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq)]
pub struct Nbt<T = DynNBT> {
	pub inner: T,
}

#[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq)]
pub struct AnonymousNbt<T = DynNBT> {
	pub inner: T,
}

impl<T: for<'de> Deserialize<'de>> MCPRead for Nbt<T> {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input_after, value): (_, T) =
			craftflow_nbt::from_slice(input).map_err(|e| Error::InvalidData(e.to_string()))?;

		// crazy shit here because we need a mutable slice from the immutable one
		let offset = input_after.as_ptr() as usize - input.as_ptr() as usize;
		let input = &mut input[offset..];

		Ok((input, Self { inner: value }))
	}
}
impl<T: Serialize> MCPWrite for Nbt<T> {
	fn write(&self, output: &mut impl Write) -> Result<usize> {
		Ok(craftflow_nbt::to_writer(output, &self.inner)
			.map_err(|e| Error::InvalidData(e.to_string()))?)
	}
}

impl<T: for<'de> Deserialize<'de>> MCPRead for AnonymousNbt<T> {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input_after, (name, value)): (_, (_, T)) = craftflow_nbt::from_slice_named(input)
			.map_err(|e| Error::InvalidData(e.to_string()))?;
		assert_eq!(name.as_ref(), "");

		// crazy shit here because we need a mutable slice from the immutable one
		let offset = input_after.as_ptr() as usize - input.as_ptr() as usize;
		let input = &mut input[offset..];

		Ok((input, Self { inner: value }))
	}
}
impl<T: Serialize> MCPWrite for AnonymousNbt<T> {
	fn write(&self, output: &mut impl Write) -> Result<usize> {
		Ok(craftflow_nbt::to_writer_named(output, &"", &self.inner)
			.map_err(|e| Error::InvalidData(e.to_string()))?)
	}
}
