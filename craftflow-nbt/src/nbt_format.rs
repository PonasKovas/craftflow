use crate::Result;
use bytes::Bytes;

pub(crate) trait NbtFormat: Sized {
	/// if the type is statically sized, this is how many bytes it is.
	/// otherwise this is set to 0 and you can use count_bytes()
	const CONST_SIZE: usize = 0;

	/// Validates (and optimizes if possible) the bytes. You can call get() after this.
	fn validate(data: &mut &mut [u8]) -> Result<()>;
	/// Splits off Self from the bytes. Must only be called on validated bytes.
	unsafe fn get(data: &mut Bytes) -> Self;
	/// Counts how many bytes this element takes. Only for validated bytes.
	/// slice must start with this element, but may have arbitrary more trailing bytes
	unsafe fn count_bytes(data: &[u8]) -> usize;
	// serializes to a buffer
	fn write(data: &[u8], output: &mut Vec<u8>) -> usize;

	fn write_self(&self, output: &mut Vec<u8>) -> usize;
}
