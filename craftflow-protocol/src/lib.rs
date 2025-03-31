#![doc(
	html_favicon_url = "https://github.com/PonasKovas/craftflow/blob/master/assets/icon.png?raw=true"
)]
#![doc(
	html_logo_url = "https://github.com/PonasKovas/craftflow/blob/master/assets/icon.png?raw=true"
)]

mod datatypes;
mod error;
pub use craftflow_nbt;
pub use error::{Error, Result};

/// Trait for writing a packet.
pub trait PacketWrite {
	fn packet_write(&self, output: &mut Vec<u8>, protocol_version: u32) -> usize;
}

/// Trait for reading a packet.
pub trait PacketRead<'a>: Sized {
	fn packet_read(input: &mut &'a [u8], protocol_version: u32) -> Result<Self>;
}

/// Trait for packet builders.
pub trait PacketBuilder {
	type Packet;

	const VERSIONS: &'static [u32];

	fn new(protocol_version: u32) -> Self;
}

// this macro is used in the generated code to define structures
//
// the reason why im including! it is because i dont want the code in this file but if i
// put it into a module idk how to access it then without making it public
// (average rust macro slop situation)
include! {"mcp_macro.rs"}

// The generated code by build.rs
#[allow(clippy::manual_range_patterns)]
#[allow(clippy::empty_line_after_doc_comments)]
mod generated {
	include!(concat!(env!("OUT_DIR"), "/generated.rs"));
}
pub use generated::*;
