use crate::{Error, MCPBaseRead, MCPBaseWrite};
use crab_nbt::serde::{de::from_bytes_unnamed, ser::to_bytes_unnamed};
use serde::{de::DeserializeOwned, Serialize};
use std::io::Write;

#[derive(Debug, Clone, PartialEq)]
pub struct Nbt<T>(pub T);

pub type DynNbt = crab_nbt::Nbt;

impl<T: DeserializeOwned> MCPBaseRead for Nbt<T> {
	fn read(_protocol_version: u32, input: &[u8]) -> crate::Result<(&[u8], Self)> {
		// let mut bytes: BytesMut = input.into();
		// match from_bytes_unnamed(&mut bytes) {
		// 	Ok(d) => {
		// 		let input = &input[(input.len() - bytes.remaining())..];
		// 		Ok((input, Nbt(d)))
		// 	}
		// 	Err(e) => Err(Error::InvalidData(format!("invalid Nbt: {e}"))),
		// }
		todo!()
	}
}

impl<T: Serialize> MCPBaseWrite for Nbt<T> {
	fn write(&self, _protocol_version: u32, output: &mut impl Write) -> crate::Result<usize> {
		// match to_bytes_unnamed(&self.0) {
		// 	Ok(bytes) => {
		// 		output.write_all(&*bytes)?;
		// 		Ok(bytes.len())
		// 	}
		// 	Err(e) => Err(Error::InvalidData(format!("invalid Nbt: {e}"))),
		// }
		todo!()
	}
}

impl MCPBaseRead for DynNbt {
	fn read(_protocol_version: u32, mut input: &[u8]) -> crate::Result<(&[u8], Self)> {
		match DynNbt::read_unnamed(&mut input) {
			Ok(nbt) => Ok((input, nbt)),
			Err(e) => Err(Error::InvalidData(format!("invalid Nbt: {e}"))),
		}
	}
}

impl MCPBaseWrite for DynNbt {
	fn write(&self, _protocol_version: u32, output: &mut impl Write) -> crate::Result<usize> {
		let bytes = self.write_unnamed();
		output.write_all(&*bytes)?;
		Ok(bytes.len())
	}
}
