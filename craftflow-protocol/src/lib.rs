#![doc(
	html_favicon_url = "https://github.com/PonasKovas/craftflow/blob/master/assets/icon.png?raw=true"
)]
#![doc(
	html_logo_url = "https://github.com/PonasKovas/craftflow/blob/master/assets/icon.png?raw=true"
)]

pub mod datatypes;
mod error;

pub use error::{Error, Result};

/// Trait for writing a packet
pub trait PacketWrite {
	fn packet_write(&self, output: &mut Vec<u8>, protocol_version: u32) -> usize;
}

/// Trait for reading a packet
pub trait PacketRead<'a>: Sized {
	fn packet_read(input: &mut &'a [u8], protocol_version: u32) -> Result<Self>;
}

/// The main internal trait that allows to write data
trait MCPWrite {
	fn mcp_write(&self, output: &mut Vec<u8>) -> usize;
}

/// The main internal trait that allows to read data
trait MCPRead<'a>: Sized {
	fn mcp_read(input: &mut &'a [u8]) -> Result<Self>;
}

// this macro is used in the generated code to define structures
include! {"mcp_macro.rs"}

// The generated code by build.rs
include!(concat!(env!("OUT_DIR"), "/generated.rs"));
