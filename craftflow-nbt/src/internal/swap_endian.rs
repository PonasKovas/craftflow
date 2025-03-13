#[cfg(target_endian = "big")]
pub fn swap_endian(slice: &mut [u8], element_size: usize) {}

#[cfg(target_endian = "little")]
pub fn swap_endian(slice: &mut [u8], element_size: usize) {
	#[cfg(not(feature = "nightly"))]
	for chunk in slice.chunks_mut(element_size) {
		chunk.reverse();
	}

	#[cfg(feature = "nightly")]
	simd(slice, element_size)
}

#[cfg(feature = "nightly")]
fn simd(slice: &mut [u8], element_size: usize) {
	use std::simd::Simd;

	let shuffle = match element_size {
		1 => return,
		2 => Simd::from_array([1, 0, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14]),
		4 => Simd::from_array([3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, 15, 14, 13, 12]),
		8 => Simd::from_array([7, 6, 5, 4, 3, 2, 1, 0, 15, 14, 13, 12, 11, 10, 9, 8]),
		_ => unreachable!(),
	};

	for chunk in slice.chunks_mut(16) {
		if chunk.len() == 16 {
			let data = Simd::<u8, 16>::from_slice(chunk);
			data.scatter(chunk, shuffle);
		} else {
			// fallback for leftover bytes
			for element in chunk.chunks_mut(element_size) {
				element.reverse();
			}
		}
	}
}
