//! Common datatypes found all throughout the network protocol.
//!

mod bool;
mod byte_array;
mod integers;
mod option;
mod string;
pub mod text;
mod varint;
mod vec;

pub use text::Text;
pub use varint::VarInt;
