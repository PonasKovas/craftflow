use crate::{Error, Result};
use std::fmt::Display;

/// NBT value tag
///
/// Most likely you don't need to use it directly. Only exported because it's used in the [`Error`] type
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tag {
	End = 0,
	Byte = 1,
	Short = 2,
	Int = 3,
	Long = 4,
	Float = 5,
	Double = 6,
	ByteArray = 7,
	String = 8,
	List = 9,
	Compound = 10,
	IntArray = 11,
	LongArray = 12,
}

impl Tag {
	/// Constructs a new [`Tag`] from a raw byte
	pub fn new(tag: u8) -> Result<Tag> {
		match tag {
			0 => Ok(Tag::End),
			1 => Ok(Tag::Byte),
			2 => Ok(Tag::Short),
			3 => Ok(Tag::Int),
			4 => Ok(Tag::Long),
			5 => Ok(Tag::Float),
			6 => Ok(Tag::Double),
			7 => Ok(Tag::ByteArray),
			8 => Ok(Tag::String),
			9 => Ok(Tag::List),
			10 => Ok(Tag::Compound),
			11 => Ok(Tag::IntArray),
			12 => Ok(Tag::LongArray),
			_ => Err(Error::InvalidTag(tag)),
		}
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
