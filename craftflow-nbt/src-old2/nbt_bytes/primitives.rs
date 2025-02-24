use crate::{
	Error, Result,
	bytes_abstr::{BytesAbstr, BytesMutAbstr},
	nbt_bytes::NbtBytes,
};
use std::mem::size_of;
use typenum::{U1, U2, U4, U8};

macro_rules! impl_primitive {
	($name:ty, $size:ty) => {
		const _: () = {
			const SIZE: usize = size_of::<$name>();

			fn get(data: &[u8]) -> $name {
				let mut bytes = [0u8; SIZE];
				bytes.copy_from_slice(&data[..SIZE]);

				<$name>::from_be_bytes(bytes)
			}

			impl<T: BytesAbstr> NbtBytes<T> for $name {
				type ConstSize = $size;

				fn validate<B: BytesMutAbstr<Immutable = T>>(data: &mut B) -> Result<Self> {
					if data.len() < SIZE {
						return Err(Error::InsufficientData(SIZE - data.len()));
					}

					let s = get(&data);
					data.advance(SIZE);

					Ok(s)
				}
				unsafe fn new(data: &mut T) -> Self {
					let s = get(&data);

					data.advance(SIZE);

					s
				}
				fn write(&self, output: &mut Vec<u8>) -> usize {
					output.extend_from_slice(&self.to_be_bytes());

					SIZE
				}
			}
		};
	};
}
impl_primitive!(i8, U1);
impl_primitive!(i16, U2);
impl_primitive!(i32, U4);
impl_primitive!(i64, U8);
impl_primitive!(f32, U4);
impl_primitive!(f64, U8);
