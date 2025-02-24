use super::{AbstrBytesMut, NbtGet, NbtValidate};
use crate::{Error, Result};
use std::marker::PhantomData;
use typenum::{U0, Unsigned};

pub(crate) struct SeqBytes<B, T> {
	pub(crate) bytes: B,
	/// invariant just for the sake of it.
	/// doesnt really matter at all in reality since we are not using this with any subtypable types
	_phantom: PhantomData<fn(T) -> T>,
}

impl<B: AbstrBytesMut, T: NbtValidate> NbtValidate for SeqBytes<B, T> {
	const IS_STATIC: bool = false;
	type StaticSize = U0;

	fn dynamic_validate<B2: AbstrBytesMut>(data: &mut B2) -> Result<B2::Immutable> {
		if data.len() < 4 {
			return Err(Error::InsufficientData(4 - data.len()));
		}
		let n_elements = i32::from_be_bytes([data[0], data[1], data[2], data[3]]) as usize;
		data.advance(4);

		if T::IS_STATIC {
			// first check if enough bytes
			let len = n_elements * T::StaticSize::USIZE;
			if data.len() < len {
				return Err(Error::InsufficientData(len - data.len()));
			}

			if n_elements > 1 {
				// also gotta make as much of it aligned. another clever hack here
				// basically we will attempt to shift the bulk of the bytes in-place to make as many aligned
				// for example, for unaligned two-byte integer sequence
				// [<1 2> <3 4> <5 6>] -> [<5> <1 2> <3 4> <6>]
				//    |     |     |           |     |     |
				// essentially splitting the last element and moving part of it to the start
				let shift_right = data.as_ptr().align_offset(T::StaticSize::USIZE);

				// save the beginning of the last element that will be overwritten
				let mut split_part = [0u8; align_of::<i64>() - 1]; // we will not need bigger than this
				let last_el_start = (n_elements - 2) * T::StaticSize::USIZE;
				split_part[..shift_right]
					.copy_from_slice(&data[last_el_start..(last_el_start + shift_right)]);

				// shift all the bytes
				data.copy_within(..(last_el_start + shift_right), shift_right);

				// write the first part of the last element to the start
				data[..shift_right].copy_from_slice(&split_part[..shift_right]);
			}

			Ok(data.split_bytes(len).freeze())
		} else {
			// dynamic elements, gotta validate each individually
			let mut bytes = data.deref_mut();
			for _ in 0..n_elements {
				T::dynamic_validate(&mut bytes)?;
			}

			// see how many bytes validation consumed
			let left = bytes.len();
			let len = data.len() - left;

			Ok(data.split_bytes(len).freeze())
		}
	}
}

impl<B: AbstrBytesMut, T: NbtGet<B>> NbtGet<B> for SeqBytes<B, T> {
	unsafe fn get(data: &mut B) -> Self {
		let n_elements = unsafe { i32::get(data) } as usize;

		if T::IS_STATIC {
		} else {
		}
	}
}

// impl<B: BytesAbstr, T: NbtBytes<B>> NbtBytes<B> for SeqBytes<T, B> {
// 	type ConstSize = U0;

// 	fn validate<B3: BytesMutAbstr<Immutable = B>>(data: &mut B3) -> Result<Self> {
// 		if data.len() < 4 {
// 			return Err(Error::InsufficientData(4 - data.len()));
// 		}
// 		let n_elements = i32::from_be_bytes([data[0], data[1], data[2], data[3]]) as usize;

// 		if T::ConstSize::USIZE == 0 {
// 			// dynamic elements, gotta validate each individually
// 			let mut bytes = data.deref_mut();
// 			for _ in 0..n_elements {
// 				T::validate(&mut bytes)?;
// 			}
// 		} else {
// 		}

// 		todo!()
// 	}
// 	unsafe fn new(data: &mut B) -> Self {
// 		todo!()
// 	}
// 	fn write(&self, output: &mut Vec<u8>) -> usize {
// 		todo!()
// 	}
// }
