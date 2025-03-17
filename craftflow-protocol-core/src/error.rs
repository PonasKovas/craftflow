use thiserror::Error;

/// The result type used in this crate
pub type Result<T> = std::result::Result<T, Error>;

/// The error type used in this crate.
#[derive(Error, Debug)]
pub enum Error {
	#[error("not enough bytes (at least {0} more needed)")]
	NotEnoughData(usize),
	#[error("VarInt too big")]
	VarIntTooBig,
	#[error("VarLong too big")]
	VarLongTooBig,
}
