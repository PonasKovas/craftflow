#![doc(
	html_favicon_url = "https://github.com/PonasKovas/craftflow/blob/master/assets/icon.png?raw=true"
)]
#![doc(
	html_logo_url = "https://github.com/PonasKovas/craftflow/blob/master/assets/icon.png?raw=true"
)]

mod datatypes;
mod error;
mod limits;
pub use craftflow_nbt;
pub use error::{Error, Result};
pub use maxlen;

/// Trait for writing a packet
pub trait PacketWrite {
	fn packet_write(&self, output: &mut Vec<u8>, protocol_version: u32) -> usize;
}

/// Trait for reading a packet
pub trait PacketRead<'a>: Sized {
	fn packet_read(input: &mut &'a [u8], protocol_version: u32) -> Result<Self>;
}

// this macro is used in the generated code to define structures
//
// the reason why im including! it is because i dont want the code in this file but if i
// put it into a module idk how to access it then without making it public
// (average rust macro slop situation)
include! {"mcp_macro.rs"}

// The generated code by build.rs
include!(concat!(env!("OUT_DIR"), "/generated.rs"));
