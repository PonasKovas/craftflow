//! Common datatypes found all throughout the network protocol.
//!

mod array;
mod buffer;
mod json;
mod nbt;
mod option;
mod primitives;
mod rest_buffer;
mod string;
mod varint;
mod varlong;

pub use array::Array;
pub use buffer::Buffer;
pub use json::Json;
pub use nbt::{NamedNbt, Nbt};
pub use rest_buffer::RestBuffer;
pub use varint::VarInt;
pub use varlong::VarLong;

// Helper functions for implementations:
////////////////////////////////////////

/// 😊
fn advance<'a>(s: &mut &'a [u8], n: usize) -> &'a [u8] {
	let (l, r) = std::mem::take(s).split_at(n);
	*s = r;
	l
}
