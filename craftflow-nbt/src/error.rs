use crate::Tag;
use thiserror::Error;

/// The result type used in this crate
pub type Result<T> = std::result::Result<T, Error>;

/// The error type used in this crate.
///
/// Either IO Error or invalid data
#[derive(Error, Debug)]
pub enum Error {
	#[error("not enough bytes (at least {0} more needed)")]
	NotEnoughData(usize),
	#[error("string not valid modified cesu-8")]
	InvalidString(#[from] simd_cesu8::DecodingError),
	#[error("invalid nbt tag {0}")]
	InvalidTag(u8),
	#[error("unexpected nbt tag {0}")]
	UnexpectedTag(Tag),
	#[error("unexpected nbt tag for {field_name:?}: expected {expected}, found {found}")]
	WrongTag {
		field_name: &'static str,
		expected: Tag,
		found: Tag,
	},
	#[error("invalid length {0}")]
	InvalidLength(i32),
	#[error("key collision in compound {0:?}")]
	KeyCollision(String),
	#[error("keys not found in compound {0:?}")]
	MissingKeys(Vec<&'static str>),
}
