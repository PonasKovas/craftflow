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
	InvalidArrayLength(i128),
	#[error("array too long ({length}), limit is {max}")]
	ArrayTooLong { length: usize, max: usize },
	#[error("{0}")]
	InvalidNbt(#[from] craftflow_nbt::Error),
	#[error("invalid enum tag {tag} in {enum_name}")]
	InvalidEnumTag { tag: i64, enum_name: &'static str },
	#[error("wrong packet id {found}, expected {expected}")]
	WrongPacketId { found: u32, expected: u32 },
	#[error("unwknown packet id {id}, state {state}")]
	UnknownPacketId {
		id: u32,
		protocol_version: u32,
		state: &'static str,
	},
}
