use thiserror::Error;

/// The result type used in this crate
pub type Result<T> = std::result::Result<T, Error>;

/// The error type used in this crate.
///
/// Either IO Error or invalid data
#[derive(Error, Debug)]
pub enum Error {
	#[error("invalid NBT tag {0}")]
	InvalidTag(u8),
	#[error("insufficient data, expected {0} more")]
	InsufficientData(usize),
	#[error("unexpected TAG_END (usually indicates a not present optional value)")]
	UnexpectedNone,
	#[error("invalid modified-cesu8 string data")]
	InvalidStringData(#[from] simd_cesu8::DecodingError),
	#[error("unexpected TAG_END as type of list or compound")]
	UnexpectedTagEnd,
}
