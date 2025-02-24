use crate::{
	abstr_bytes::AbstrBytes,
	nbt::{NbtGet, NbtValidate, NbtWrite},
};
use std::mem::size_of;
use typenum::{U1, U2, U4, U8};

macro_rules! impl_primitive {
	($name:ty, $size:ty) => {
		const _: () = {
			const _: () = {
				if align_of::<$name>() != size_of::<$name>() {
					panic!("primitive size must be equal to its alignment");
				}
			};
			const SIZE: usize = size_of::<$name>();

			impl NbtValidate for $name {
				const IS_STATIC: bool = true;
				type StaticSize = $size;
			}
			impl<B: AbstrBytes> NbtGet<B> for $name {
				unsafe fn get(data: &mut B) -> Self {
					let mut bytes = [0u8; SIZE];
					bytes.copy_from_slice(&data[..SIZE]);
					data.advance(SIZE);

					<$name>::from_be_bytes(bytes)
				}
			}
			impl NbtWrite for $name {
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
