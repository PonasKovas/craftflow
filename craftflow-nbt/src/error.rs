use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
	#[error("IO error: {0}")]
	IOError(#[from] std::io::Error),
	#[error("Invalid data: {0}")]
	InvalidData(String),
}

impl serde::ser::Error for Error {
	fn custom<T: std::fmt::Display>(msg: T) -> Self {
		Error::InvalidData(msg.to_string())
	}
}

impl serde::de::Error for Error {
	fn custom<T: std::fmt::Display>(msg: T) -> Self {
		Error::InvalidData(msg.to_string())
	}
}
