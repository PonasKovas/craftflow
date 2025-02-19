use crate::{nbt_format::NbtFormat, Error, Result};
use bytes::{Buf, Bytes};
use std::fmt::Display;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Tag {
	End = 0,
	Byte,
	Short,
	Int,
	Long,
	Float,
	Double,
	ByteArray,
	String,
	List,
	Compound,
	IntArray,
	LongArray,
}

impl NbtFormat for Tag {
	const CONST_SIZE: usize = 1;

	unsafe fn get(data: &mut Bytes) -> Self {
		unsafe { Self::new(data.get_u8()).unwrap_unchecked() }
	}
	fn write(&self, output: &mut Vec<u8>) -> usize {
		output.push(*self as u8);

		1
	}
	fn validate(data: &mut &mut [u8]) -> Result<()> {
		if data.len() < 1 {
			return Err(Error::InsufficientData(1));
		}

		// make sure its a valid tag
		Self::new(data[0])?;

		Ok(())
	}
	unsafe fn count_bytes(_data: &[u8]) -> usize {
		1
	}
}

impl Tag {
	/// Constructs a new [`Tag`] from a raw byte
	pub fn new(tag: u8) -> Result<Tag> {
		Ok(match tag {
			0 => Tag::End,
			1 => Tag::Byte,
			2 => Tag::Short,
			3 => Tag::Int,
			4 => Tag::Long,
			5 => Tag::Float,
			6 => Tag::Double,
			7 => Tag::ByteArray,
			8 => Tag::String,
			9 => Tag::List,
			10 => Tag::Compound,
			11 => Tag::IntArray,
			12 => Tag::LongArray,
			_ => return Err(Error::InvalidTag(tag)),
		})
	}
}

impl Display for Tag {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Tag::End => "End",
				Tag::Byte => "Byte",
				Tag::Short => "Short",
				Tag::Int => "Int",
				Tag::Long => "Long",
				Tag::Float => "Float",
				Tag::Double => "Double",
				Tag::ByteArray => "ByteArray",
				Tag::String => "String",
				Tag::List => "List",
				Tag::Compound => "Compound",
				Tag::IntArray => "IntArray",
				Tag::LongArray => "LongArray",
			}
		)
	}
}
