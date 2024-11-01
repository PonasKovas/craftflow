#![doc(
	html_favicon_url = "https://github.com/PonasKovas/craftflow/blob/master/assets/icon.png?raw=true"
)]
#![doc(
	html_logo_url = "https://github.com/PonasKovas/craftflow/blob/master/assets/icon.png?raw=true"
)]

mod into_traits;
mod packet_read_write;
mod supported_versions;

// Macros that are used to generate packets and types with automatic
// MCPRead, MCPWrite trait implementations
include!("macros.rs");

#[cfg(test)]
mod test_prompt_example;

pub use into_traits::*;
pub use packet_read_write::*;
pub use supported_versions::*;

// The generated code by build.rs
include!(concat!(env!("OUT_DIR"), "/generated.rs"));
