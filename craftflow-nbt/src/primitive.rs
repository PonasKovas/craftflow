use crate::{nbt_format::NbtFormat, Error, Result};
use bytes::{Buf, Bytes};
use std::mem::size_of;

// some basic implementations for primitives
macro_rules! impl_primitive {
	($name:ty) => {
		impl NbtFormat for $name {
			const CONST_SIZE: usize = std::mem::size_of::<$name>();

			unsafe fn get(data: &mut Bytes) -> Self {
				const SIZE: usize = std::mem::size_of::<$name>();
				let mut bytes = [0u8; SIZE];
				bytes.copy_from_slice(&data[..SIZE]);
				data.advance(SIZE);

				<$name>::from_be_bytes(bytes)
			}
			fn write(&self, output: &mut Vec<u8>) -> usize {
				output.extend_from_slice(&self.to_be_bytes());

				size_of::<$name>()
			}
			fn validate(data: &mut &mut [u8]) -> Result<()> {
				const SIZE: usize = std::mem::size_of::<$name>();
				if data.len() < SIZE {
					return Err(Error::InsufficientData(SIZE - data.len()));
				}

				Ok(())
			}
			unsafe fn count_bytes(_data: &[u8]) -> usize {
				std::mem::size_of::<$name>()
			}
		}
	};
}
impl_primitive!(i8);
impl_primitive!(i16);
impl_primitive!(i32);
impl_primitive!(i64);
impl_primitive!(f32);
impl_primitive!(f64);
