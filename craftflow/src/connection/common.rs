pub fn varint_num_bytes(varint: i32) -> usize {
	let value = varint as u32;
	if value == 0 {
		return 1;
	}
	let bits_needed = 32 - value.leading_zeros();
	bits_needed.div_ceil(7) as usize
}
