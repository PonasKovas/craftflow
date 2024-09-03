//! Prefixes the inner type with a boolean, indicating whether the value is present or not.

use crate::{MCPReadable, MCPWritable};
use std::io::{Read, Write};

impl<T: MCPReadable> MCPReadable for Option<T> {
	fn read(source: &mut impl Read) -> anyhow::Result<Self> {
		if bool::read(source)? {
			Ok(Some(T::read(source)?))
		} else {
			Ok(None)
		}
	}
}

impl<T: MCPWritable> MCPWritable for Option<T> {
	fn write(&self, to: &mut impl Write) -> anyhow::Result<usize> {
		match self {
			Some(value) => {
				bool::write(&true, to)?;
				value.write(to)
			}
			None => bool::write(&false, to),
		}
	}
}
