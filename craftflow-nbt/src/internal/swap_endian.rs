#[cfg(target_endian = "big")]
#[inline]
pub fn swap_endian(slice: &mut [u8], element_size: usize) {}

#[cfg(target_endian = "little")]
#[inline]
pub fn swap_endian(slice: &mut [u8], element_size: usize) {
	if element_size == 1 {
		return;
	}

	#[cfg(not(feature = "nightly"))]
	match element_size {
		2 => scalar::<2>(slice),
		4 => scalar::<4>(slice),
		8 => scalar::<8>(slice),
		_ => unreachable!(),
	}

	#[cfg(feature = "nightly")]
	match element_size {
		2 => simd::<2>(slice),
		4 => simd::<4>(slice),
		8 => simd::<8>(slice),
		_ => unreachable!(),
	}
}

#[inline]
fn scalar<const N: usize>(slice: &mut [u8]) {
	for chunk in slice.chunks_mut(N) {
		chunk.reverse();
	}
}

#[cfg(feature = "nightly")]
#[inline]
fn simd<const N: usize>(slice: &mut [u8]) {
	use std::simd::{Simd, simd_swizzle};

	// if aligned to N, we can do SIMD directly, and it will not split an element
	if slice.as_ptr() as usize % N == 0 {
		let (start, simd, end) = slice.as_simd_mut::<16>();
		debug_assert_eq!(start.len() % N, 0);
		debug_assert_eq!(end.len() % N, 0);
		scalar::<N>(start);
		scalar::<N>(end);
		for simd in simd {
			*simd = if N == 2 {
				simd_swizzle!(
					*simd,
					[1, 0, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14]
				)
			} else if N == 4 {
				simd_swizzle!(
					*simd,
					[3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, 15, 14, 13, 12]
				)
			} else if N == 8 {
				simd_swizzle!(
					*simd,
					[7, 6, 5, 4, 3, 2, 1, 0, 15, 14, 13, 12, 11, 10, 9, 8]
				)
			} else {
				unreachable!()
			}
		}
	} else {
		// if not aligned to N, we have to resort to copying
		for chunk in slice.chunks_mut(16) {
			if chunk.len() < 16 {
				scalar::<N>(chunk);
				continue;
			}
			let simd: Simd<u8, 16> = Simd::from_slice(chunk);

			chunk.copy_from_slice(
				&if N == 2 {
					simd_swizzle!(simd, [1, 0, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14])
				} else if N == 4 {
					simd_swizzle!(simd, [3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, 15, 14, 13, 12])
				} else if N == 8 {
					simd_swizzle!(simd, [7, 6, 5, 4, 3, 2, 1, 0, 15, 14, 13, 12, 11, 10, 9, 8])
				} else {
					unreachable!()
				}[..],
			);
		}
	}
}
