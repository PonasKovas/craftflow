use super::{MCP, MCPRead, MCPWrite};
use crate::Result;

type Pos = (i32, i16, i32);

const BITS_X: u32 = 26;
const BITS_Y: u32 = 12;
const BITS_Z: u32 = 26;

const MASK_X: u64 = (1u64 << BITS_X) - 1;
const MASK_Y: u64 = (1u64 << BITS_Y) - 1;
const MASK_Z: u64 = (1u64 << BITS_Z) - 1;

const V5_SHIFT_X: u32 = BITS_Y + BITS_Z;
const V5_SHIFT_Y: u32 = BITS_Z;
const V5_SHIFT_Z: u32 = 0;

const V477_SHIFT_X: u32 = BITS_Y + BITS_Z;
const V477_SHIFT_Z: u32 = BITS_Y;
const V477_SHIFT_Y: u32 = 0;

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct PositionV5;

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct PositionV477;

impl MCP for PositionV5 {
	type Data = Pos;
}
impl<'a> MCPRead<'a> for PositionV5 {
	fn mcp_read(input: &mut &[u8]) -> Result<Pos> {
		let packed = i64::mcp_read(input)?;

		let x = packed >> V5_SHIFT_X;

		// Shift bits left so its MSB aligns with the i64's MSB.
		let y_shifted_left = packed << (64 - BITS_Y - V5_SHIFT_Y);
		// Arithmetic right shift back to perform sign extension based on original MSB.
		let y = y_shifted_left >> (64 - BITS_Y);

		// same as for Y
		let z_shifted_left = packed << (64 - BITS_Z - V5_SHIFT_Z);
		let z = z_shifted_left >> (64 - BITS_Z);

		Ok((x as i32, y as i16, z as i32))
	}
}
impl MCPWrite for PositionV5 {
	fn mcp_write(data: &Pos, output: &mut Vec<u8>) -> usize {
		let x_masked = (data.0 as i64) & (MASK_X as i64);
		let y_masked = (data.1 as i64) & (MASK_Y as i64);
		let z_masked = (data.2 as i64) & (MASK_Z as i64);

		let packed = (x_masked << V5_SHIFT_X) | (y_masked << V5_SHIFT_Y) | (z_masked << V5_SHIFT_Z);

		i64::mcp_write(&packed, output)
	}
}

impl MCP for PositionV477 {
	type Data = Pos;
}

impl<'a> MCPRead<'a> for PositionV477 {
	fn mcp_read(input: &mut &'a [u8]) -> Result<Pos> {
		let packed = i64::mcp_read(input)?;

		let x = packed >> V477_SHIFT_X;

		// Shift bits left so its MSB aligns with the i64's MSB.
		let y_shifted_left = packed << (64 - BITS_Y - V477_SHIFT_Y);
		// Arithmetic right shift back to perform sign extension based on original MSB.
		let y = y_shifted_left >> (64 - BITS_Y);

		// same as for Y
		let z_shifted_left = packed << (64 - BITS_Z - V477_SHIFT_Z);
		let z = z_shifted_left >> (64 - BITS_Z);

		Ok((x as i32, y as i16, z as i32))
	}
}

impl MCPWrite for PositionV477 {
	fn mcp_write(data: &Self::Data, output: &mut Vec<u8>) -> usize {
		let x_masked = (data.0 as i64) & (MASK_X as i64);
		let y_masked = (data.1 as i64) & (MASK_Y as i64);
		let z_masked = (data.2 as i64) & (MASK_Z as i64);

		let packed =
			(x_masked << V477_SHIFT_X) | (y_masked << V477_SHIFT_Y) | (z_masked << V477_SHIFT_Z);

		i64::mcp_write(&packed, output)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const TEST_CASES: &[(i32, i16, i32)] = &[
		(0, 0, 0),
		(33554431, 2047, 33554431),
		(-33554432, -2048, -33554432),
		(1, 1, 1),
		(-1, -1, -1),
		(1000, 100, 1000),
		(-1000, -100, -1000),
		(33554430, 2046, 33554430),
		(-33554431, -2047, -33554431),
		(1000000, 500, 1000000),
		(-1000000, -500, -1000000),
		(12345678, -987, 12345678),
		(-12345678, 987, -12345678),
		(5000000, 2047, 5000000),
		(-5000000, -2048, -5000000),
		(0, -2048, 0),
		(1000000, 1024, 1000000),
		(-1000000, -1024, -1000000),
	];

	#[test]
	fn position_v5_roundtrip() {
		for (i, case) in TEST_CASES.into_iter().enumerate() {
			let mut buf = Vec::new();

			PositionV5::mcp_write(case, &mut buf);
			let result = PositionV5::mcp_read(&mut &buf[..]).unwrap();
			assert_eq!(*case, result, "{i}");
		}
	}

	#[test]
	fn position_v477_roundtrip() {
		for (i, case) in TEST_CASES.into_iter().enumerate() {
			let mut buf = Vec::new();

			PositionV477::mcp_write(case, &mut buf);
			let result = PositionV477::mcp_read(&mut &buf[..]).unwrap();
			assert_eq!(*case, result, "{i}");
		}
	}
}
