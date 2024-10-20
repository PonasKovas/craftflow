use std::fmt::Display;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tag {
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

impl Tag {
	/// Returns `true` if `self` can be used where `other` is expected.
	///
	/// For example, `Byte` can be used where `Int` is expected, but not the other way around.
	pub fn compatible_with(&self, other: &Self) -> bool {
		// short circuit for equal types
		if self == other {
			return true;
		}

		let c: &[Tag] = match self {
			Tag::Byte => &[Tag::Short, Tag::Int, Tag::Long],
			Tag::Short => &[Tag::Int, Tag::Long],
			Tag::Int => &[Tag::Long],
			Tag::Float => &[Tag::Double],
			Tag::ByteArray => &[Tag::List, Tag::IntArray, Tag::LongArray],
			Tag::IntArray => &[Tag::List, Tag::LongArray],
			Tag::LongArray => &[Tag::List],
			_ => &[],
		};

		c.contains(other)
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
