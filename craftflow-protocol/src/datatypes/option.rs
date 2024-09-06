//! Prefixes the inner type with a boolean, indicating whether the value is present or not.

use crate::MinecraftProtocol;
use std::io::{Read, Write};

impl<T: MinecraftProtocol> MinecraftProtocol for Option<T> {
	fn read(protocol_version: u32, source: &mut impl Read) -> anyhow::Result<Self> {
		if bool::read(protocol_version, source)? {
			Ok(Some(T::read(protocol_version, source)?))
		} else {
			Ok(None)
		}
	}
	fn write(&self, protocol_version: u32, to: &mut impl Write) -> anyhow::Result<usize> {
		match self {
			Some(value) => {
				bool::write(&true, protocol_version, to)?;
				value.write(protocol_version, to)
			}
			None => bool::write(&false, protocol_version, to),
		}
	}
}
