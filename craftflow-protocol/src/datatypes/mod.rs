//! Common datatypes found all throughout the network protocol.
//!

mod bool;
mod boxed_slice;
mod integers;
mod json;
mod nbt;
mod option;
mod string;
pub mod text;
mod varint;
mod vec;

pub use json::Json;
pub use nbt::{DynNbt, Nbt};
pub use text::Text;
pub use varint::VarInt;

pub use crab_nbt::nbt;
