use bytemuck::cast_slice_mut;
use std::simd::{num::SimdUint, Simd, SimdElement};

#[inline]
pub(super) fn swap_endian(slice: &mut [u8], size: usize) {
	#[cfg(target_endian = "little")]
	{
		// TODO; once const expressions are a bit more implemented in rust, we can use
		// the T::CONST_SIZE directly as a constant. until then we gotta do this
		// hopefully mr compiler can still optimize this to not have runtime overhead.
		// likely related issue: https://github.com/rust-lang/rust/issues/76560
		match size {
			1 => {} // no need to swap anything :) yay
			2 => {
				implementation::<u16>(cast_slice_mut(slice));
			}
			4 => {
				implementation::<u32>(cast_slice_mut(slice));
			}
			8 => {
				implementation::<u64>(cast_slice_mut(slice));
			}
			_ => unreachable!(),
		}
	}
}

#[inline]
pub(super) fn implementation<T: SimdElement>(slice: &mut [T])
where
	[T]: SwapEndian,
	Simd<T, 64>: SimdUint,
	Simd<T, 32>: SimdUint,
	Simd<T, 16>: SimdUint,
{
	let (pre, simd, post) = slice.as_simd_mut::<64>();
	simd.into_iter().for_each(|el| *el = el.swap_bytes());

	// left-side leftovers
	let (pre, simd, post2) = pre.as_simd_mut::<32>();
	simd.into_iter().for_each(|el| *el = el.swap_bytes());
	pre.swap_endian();
	post2.swap_endian();

	// right-side leftovers
	let (pre2, simd, post) = post.as_simd_mut::<32>();
	simd.into_iter().for_each(|el| *el = el.swap_bytes());
	pre2.swap_endian();
	post.swap_endian();
}

pub(super) trait SwapEndian {
	fn swap_endian(&mut self);
}

macro_rules! swap_endianness_int {
	($prim_type:ident) => {
		impl SwapEndian for [$prim_type] {
			#[inline]
			fn swap_endian(&mut self) {
				for el in self {
					*el = el.swap_bytes();
				}
			}
		}
	};
}
swap_endianness_int!(u8);
swap_endianness_int!(u16);
swap_endianness_int!(u32);
swap_endianness_int!(u64);
