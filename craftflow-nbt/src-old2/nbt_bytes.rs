mod primitives;
mod seq;
mod string;

use crate::Result;
use crate::bytes_abstr::{BytesAbstr, BytesMutAbstr};
use generic_array::ArrayLength;

pub(crate) trait NbtBytes<T: BytesAbstr>: Sized {
	/// if the type is statically sized, this is how many bytes it is.
	/// otherwise this is set to 0
	type ConstSize: ArrayLength;

	/// Validates (and optimizes if possible) the bytes. Advances the given buffer
	fn validate<B: BytesMutAbstr<Immutable = T>>(data: &mut B) -> Result<Self>;
	/// Splits off Self from the bytes. Must only be called on already validated bytes.
	/// May not included all related bytes in the returned structure. For getting size - compare how much it advanced the input
	unsafe fn new(data: &mut T) -> Self;
	/// Serializes to a buffer and returns how many bytes were written
	fn write(&self, output: &mut Vec<u8>) -> usize;
}
