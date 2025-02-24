mod primitives;
mod seq;
mod string;

use crate::Result;
use crate::abstr_bytes::{AbstrBytes, AbstrBytesMut};
use generic_array::ArrayLength;

pub(crate) trait NbtValidate {
	/// If static validation available, use StaticSize
	const IS_STATIC: bool;
	type StaticSize: ArrayLength;
	// Otherwise use this function

	/// Validates (and optimizes if possible) the bytes. Advances the given buffer, returned buffer can be used with new()
	fn dynamic_validate<B: AbstrBytesMut>(_data: &mut B) -> Result<B::Immutable> {
		unreachable!("this type doesn't need dynamic validation")
	}
}

pub(crate) trait NbtGet<B: AbstrBytes>: Sized {
	/// Splits off Self from the bytes. Must only be called on already validated bytes.
	/// May not included all related bytes in the returned structure. For getting size - compare how much it advanced the input
	unsafe fn get(data: &mut B) -> Self;
}

pub(crate) trait NbtWrite {
	/// Serializes to a buffer and returns how many bytes were written
	fn write(&self, output: &mut Vec<u8>) -> usize;
}
