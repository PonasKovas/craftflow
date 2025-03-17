//! Common datatypes found all throughout the network protocol.
//!

// pub mod array;
// mod bool;
// mod buffer;
// mod float;
// mod integers;
// mod json;
// mod nbt;
// mod option;
// mod rest_buffer;
// mod string;
mod varint;
mod varlong;

// pub use array::Array;
// pub use buffer::Buffer;
// pub use json::Json;
// pub use nbt::{AnonymousNbt, Nbt};
// pub use rest_buffer::RestBuffer;
pub use varint::VarInt;
pub use varlong::VarLong;

use crate::Result;

fn advance(s: &mut &[u8], n: usize) {
	*s = &std::mem::take(s)[n..];
}

fn read_byte(s: &mut &[u8]) -> Result<u8> {
	const SIZE: usize = 1;
	if s.len() < SIZE {
		return Err(crate::Error::NotEnoughData(SIZE));
	}

	let r = u8::from_be_bytes([s[0]]);
	advance(s, SIZE);

	Ok(r)
}
fn read_short(s: &mut &[u8]) -> Result<u16> {
	const SIZE: usize = 2;
	if s.len() < SIZE {
		return Err(crate::Error::NotEnoughData(SIZE));
	}

	let r = u16::from_be_bytes([s[0], s[1]]);
	advance(s, SIZE);

	Ok(r)
}
fn read_int(s: &mut &[u8]) -> Result<u32> {
	const SIZE: usize = 4;
	if s.len() < SIZE {
		return Err(crate::Error::NotEnoughData(SIZE));
	}

	let r = u32::from_be_bytes([s[0], s[1], s[2], s[3]]);
	advance(s, SIZE);

	Ok(r)
}
