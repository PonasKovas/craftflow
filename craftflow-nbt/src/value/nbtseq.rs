use crate::{advance, byteswap::swap_endian, nbt_format::NbtFormat, Error};
use bytes::Bytes;
use std::marker::PhantomData;

mod sized;
// mod r#unsized;

#[derive(Clone, Debug)]
pub(crate) struct NbtSeq<T> {
	/// The "meat" of the sequence - all bytes except for the number of elements.
	data: Bytes,
	/// invariant just for the sake of it.
	/// doesnt really matter at all in reality since we are not using this with any subtypable types
	_phantom: PhantomData<fn(T) -> T>,
}

impl<T: NbtFormat> NbtFormat for NbtSeq<T> {
	fn validate(data: &mut &mut [u8]) -> crate::Result<()> {
		if data.len() < 4 {
			return Err(Error::InsufficientData(4 - data.len()));
		}
		let len = i32::from_be_bytes([data[0], data[1], data[2], data[3]]) as usize;
		advance(data, 4);

		if T::CONST_SIZE == 0 {
			// dynamic elements, gotta validate each individually
			for _ in 0..len {
				T::validate(data)?;
			}

			return Ok(());
		}

		// const sized elements

		let n_bytes = len * T::CONST_SIZE;

		if data.len() < n_bytes {
			return Err(Error::InsufficientData(n_bytes - data.len()));
		}

		// if size of each element is const, these are integers and we need to swap
		// the bytes of all of them to correct for the big-endian used in NBT
		swap_endian(&mut data[..n_bytes], T::CONST_SIZE);

		Ok(())
	}
	unsafe fn get(data: &mut Bytes) -> Self {
		let byte_len = Self::count_bytes(&data[..]);

		Self::new_raw(data.split_to(byte_len))
	}
	unsafe fn count_bytes(data: &[u8]) -> usize {
		let len = i32::from_be_bytes([data[0], data[1], data[2], data[3]]) as usize;

		let byte_len = if T::CONST_SIZE == 0 {
			// dynamic elements, gotta count the bytes
			Self::count_bytes(&data[..])
		} else {
			len * T::CONST_SIZE
		};

		byte_len + 4
	}
	fn write(&self, output: &mut Vec<u8>) -> usize {
		if T::CONST_SIZE == 0 {
			// dynamic elements, gotta write each individually
			let mut written = 0;

			written += (self.len() as i32).write(output);
			for s in self.iter() {
				written += s.write(output);
			}

			written
		} else {
			output.extend_from_slice(&*self.data);

			let written = unsafe { Self::count_bytes(&*self.data) };

			// swap the endian
			let start_pos = output.len() + 4 - written;
			swap_endian(&mut output[start_pos..], T::CONST_SIZE);

			written
		}
	}
}

impl<T: NbtFormat> NbtSeq<T> {
	pub fn len(&self) -> usize {
		i32::from_be_bytes([self.data[0], self.data[1], self.data[2], self.data[3]]) as usize
	}
	pub fn iter(&self) -> NbtSeqIter<T> {
		NbtSeqIter::new(self.data.clone(), self.len())
	}
	fn new_raw(data: Bytes) -> Self {
		Self {
			data,
			_phantom: PhantomData,
		}
	}
}
pub struct NbtSeqIter<T> {
	data: Bytes,
	elem_to_go: usize,
	_phantom: PhantomData<fn(T) -> T>,
}
impl<T: NbtFormat> NbtSeqIter<T> {
	fn new(data: Bytes, n: usize) -> Self {
		Self {
			data,
			elem_to_go: n,
			_phantom: PhantomData,
		}
	}
}
impl<T: NbtFormat> Iterator for NbtSeqIter<T> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		if self.elem_to_go == 0 {
			return None;
		}

		self.elem_to_go -= 1;

		Some(unsafe { T::get(&mut self.data) })
	}
}
