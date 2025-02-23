use bytes::{Buf, Bytes, BytesMut};
use std::ops::{Deref, DerefMut};

pub(crate) trait BytesAbstr: Deref<Target = [u8]> + Sized {
	fn advance(&mut self, n: usize);
	fn truncate(&mut self, len: usize);
	fn split_chunk(&mut self, n: usize) -> Self;
}
pub(crate) trait BytesMutAbstr: BytesAbstr + DerefMut {
	type Immutable: BytesAbstr;

	fn freeze(self) -> Self::Immutable;
}

impl BytesAbstr for Bytes {
	fn advance(&mut self, n: usize) {
		Buf::advance(self, n);
	}
	fn truncate(&mut self, len: usize) {
		self.truncate(len);
	}
	fn split_chunk(&mut self, n: usize) -> Self {
		self.split_to(n)
	}
}
impl BytesAbstr for BytesMut {
	fn advance(&mut self, n: usize) {
		Buf::advance(self, n);
	}
	fn truncate(&mut self, len: usize) {
		self.truncate(len);
	}
	fn split_chunk(&mut self, n: usize) -> Self {
		self.split_to(n)
	}
}
impl BytesMutAbstr for BytesMut {
	type Immutable = Bytes;

	fn freeze(self) -> Self::Immutable {
		self.freeze()
	}
}

impl<'a> BytesAbstr for &'a [u8] {
	fn advance(&mut self, n: usize) {
		*self = &self[n..];
	}
	fn truncate(&mut self, len: usize) {
		*self = &self[..len];
	}
	fn split_chunk(&mut self, n: usize) -> Self {
		let (start, end) = self.split_at(n);

		*self = end;

		start
	}
}
impl<'a> BytesAbstr for &'a mut [u8] {
	fn advance(&mut self, n: usize) {
		*self = &mut std::mem::take(self)[n..];
	}
	fn truncate(&mut self, len: usize) {
		*self = &mut std::mem::take(self)[..len];
	}
	fn split_chunk(&mut self, n: usize) -> Self {
		let (start, end) = std::mem::take(self).split_at_mut(n);

		*self = end;

		start
	}
}
impl<'a> BytesMutAbstr for &'a mut [u8] {
	type Immutable = &'a [u8];

	fn freeze(self) -> Self::Immutable {
		self
	}
}
