use super::{BytesAbstr, BytesMutAbstr, NbtBytes};
use crate::{Error, Result};
use std::marker::PhantomData;
use typenum::{U0, Unsigned};

pub(crate) struct SeqBytes<T, B> {
	pub(crate) bytes: B,
	/// invariant just for the sake of it.
	/// doesnt really matter at all in reality since we are not using this with any subtypable types
	_phantom: PhantomData<fn(T) -> T>,
}

impl<B: BytesAbstr, T: NbtBytes<B>> NbtBytes<B> for SeqBytes<T, B> {
	type ConstSize = U0;

	fn validate<B3: BytesMutAbstr<Immutable = B>>(data: &mut B3) -> Result<Self> {
		if data.len() < 4 {
			return Err(Error::InsufficientData(4 - data.len()));
		}
		let n_elements = i32::from_be_bytes([data[0], data[1], data[2], data[3]]) as usize;

		if T::ConstSize::USIZE == 0 {
			// dynamic elements, gotta validate each individually
			let mut bytes = data.deref_mut();
			for _ in 0..n_elements {
				T::validate(&mut bytes)?;
			}
		} else {
		}

		todo!()
	}
	unsafe fn new(data: &mut B) -> Self {
		todo!()
	}
	fn write(&self, output: &mut Vec<u8>) -> usize {
		todo!()
	}
}
