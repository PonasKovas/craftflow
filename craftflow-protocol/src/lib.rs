#![doc(
	html_favicon_url = "https://github.com/PonasKovas/craftflow/blob/master/assets/icon.png?raw=true"
)]
#![doc(
	html_logo_url = "https://github.com/PonasKovas/craftflow/blob/master/assets/icon.png?raw=true"
)]

pub mod datatypes;
mod error;

pub use error::{Error, Result};

/// The main trait that allows to write data in the **M**ine**c**raft **P**rotocol.
pub trait MCPWrite {
	fn mcp_write(&self, output: &mut Vec<u8>) -> usize;
}

/// The main trait that allows to read data in the **M**ine**c**raft **P**rotocol.
pub trait MCPRead<'a>: Sized {
	fn mcp_read(input: &mut &'a [u8]) -> Result<Self>;
}
