//! slop for converting between `Vec<T>` and `Vec<u8>` with primitives
//! And also for swapping the bytes of those slices to a different endianness

use std::mem::{ManuallyDrop, size_of};

pub(super) trait ToVecByteArray: Sized {
	fn into_vec_byte_arr(self) -> Vec<u8>;
}

macro_rules! to_vec_to_box {
	($prim_type:ty) => {
		impl ToVecByteArray for Vec<$prim_type> {
			fn into_vec_byte_arr(self) -> Vec<u8> {
				let d = ManuallyDrop::new(self);
				let ptr = d.as_ptr() as *mut u8;
				let len = d.len() * size_of::<$prim_type>();
				let cap = d.capacity() * size_of::<$prim_type>();

				unsafe { Vec::from_raw_parts(ptr, len, cap) }
			}
		}
	};
}
to_vec_to_box!(i8);
to_vec_to_box!(i16);
to_vec_to_box!(i32);
to_vec_to_box!(i64);
to_vec_to_box!(f32);
to_vec_to_box!(f64);
