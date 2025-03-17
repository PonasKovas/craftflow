use thiserror::Error;

/// The result type used in this crate
pub type Result<T> = std::result::Result<T, Error>;

/// The error type used in this crate.
#[derive(Error, Debug)]
pub enum Error {
	#[error("not enough bytes (at least {0} more needed)")]
	NotEnoughData(usize),
	#[error("varint too big")]
	VarIntTooBig,
	#[error("varlong too big")]
	VarLongTooBig,
	#[error("string too long ({length}), limit is {max}")]
	StringTooLong { length: usize, max: usize },
	#[error("string not valid utf-8")]
	StringInvalidUtf8,
	#[error("invalid array length {0}")]
	InvalidArrayLength(i64),
	#[error("array too long ({length}), limit is {max}")]
	ArrayTooLong { length: usize, max: usize },
	#[error("{0}")]
	InvalidJson(#[from] serde_json::Error),
	#[error("{0}")]
	InvalidNbt(#[from] craftflow_nbt::Error),
}
