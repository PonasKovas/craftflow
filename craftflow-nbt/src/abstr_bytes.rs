use bytes::{Buf, Bytes, BytesMut};
use std::ops::{Deref, DerefMut};

pub(crate) trait AbstrBytes: Deref<Target = [u8]> + Sized {
	fn advance(&mut self, n: usize);
	fn truncate(&mut self, len: usize);
	fn split_bytes(&mut self, n: usize) -> Self;
}
pub(crate) trait AbstrBytesMut: AbstrBytes + DerefMut {
	type Immutable: AbstrBytes;

	fn freeze(self) -> Self::Immutable;
}

impl AbstrBytes for Bytes {
	fn advance(&mut self, n: usize) {
		Buf::advance(self, n);
	}
	fn truncate(&mut self, len: usize) {
		self.truncate(len);
	}
	fn split_bytes(&mut self, n: usize) -> Self {
		self.split_to(n)
	}
}
impl AbstrBytes for BytesMut {
	fn advance(&mut self, n: usize) {
		Buf::advance(self, n);
	}
	fn truncate(&mut self, len: usize) {
		self.truncate(len);
	}
	fn split_bytes(&mut self, n: usize) -> Self {
		self.split_to(n)
	}
}
impl AbstrBytesMut for BytesMut {
	type Immutable = Bytes;

	fn freeze(self) -> Self::Immutable {
		self.freeze()
	}
}

impl<'a> AbstrBytes for &'a [u8] {
	fn advance(&mut self, n: usize) {
		*self = &self[n..];
	}
	fn truncate(&mut self, len: usize) {
		*self = &self[..len];
	}
	fn split_bytes(&mut self, n: usize) -> Self {
		let (start, end) = self.split_at(n);

		*self = end;

		start
	}
}
impl<'a> AbstrBytes for &'a mut [u8] {
	fn advance(&mut self, n: usize) {
		*self = &mut std::mem::take(self)[n..];
	}
	fn truncate(&mut self, len: usize) {
		*self = &mut std::mem::take(self)[..len];
	}
	fn split_bytes(&mut self, n: usize) -> Self {
		let (start, end) = std::mem::take(self).split_at_mut(n);

		*self = end;

		start
	}
}
impl<'a> AbstrBytesMut for &'a mut [u8] {
	type Immutable = &'a [u8];

	fn freeze(self) -> Self::Immutable {
		self
	}
}
