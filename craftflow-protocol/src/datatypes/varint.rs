use crate::Error;
use crate::Result;
use crate::{MCPRead, MCPWrite};

/// A Minecraft Protocol VarInt
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct VarInt(pub i32);

impl VarInt {
	/// Returns the length (in bytes) of the VarInt in the Minecraft Protocol format.
	pub fn num_bytes(&self) -> usize {
		let value = self.0 as u32;
		if value == 0 {
			return 1;
		}
		let bits_needed = 32 - value.leading_zeros();
		((bits_needed + 6) / 7) as usize
	}
}

impl<'a> MCPRead<'a> for VarInt {
	fn mcp_read(input: &mut &[u8]) -> Result<Self> {
		let mut num_read = 0; // Count of bytes that have been read
		let mut result = 0i32; // The VarInt being constructed

		loop {
			// VarInts are at most 5 bytes long.
			if num_read == 5 {
				return Err(Error::VarIntTooBig);
			}

			let byte = u8::mcp_read(input)?;
			let value = (byte & 0b0111_1111) as i32;
			result |= value << (7 * num_read);
			num_read += 1;

			// If the high bit is not set, this was the last byte in the VarInt
			if (byte & 0b1000_0000) == 0 {
				break;
			}
		}

		Ok(Self(result))
	}
}

impl MCPWrite for VarInt {
	fn mcp_write(&self, output: &mut Vec<u8>) -> usize {
		let mut i = 0;
		let mut value = self.0;

		loop {
			// Take the 7 lower bits of the value
			let mut temp = (value & 0b0111_1111) as u8;

			// Shift the value 7 bits to the right.
			value = ((value as u32) >> 7) as i32;

			// If there is more data to write, set the high bit
			if value != 0 {
				temp |= 0b1000_0000;
			}

			output.push(temp);
			i += 1;

			// If there is no more data to write, exit the loop
			if value == 0 {
				break;
			}
		}

		i
	}
}

impl From<VarInt> for i128 {
	fn from(value: VarInt) -> Self {
		value.0 as i128
	}
}
impl From<VarInt> for i64 {
	fn from(value: VarInt) -> Self {
		value.0 as i64
	}
}
impl From<VarInt> for i32 {
	fn from(value: VarInt) -> Self {
		value.0 as i32
	}
}
impl From<i32> for VarInt {
	fn from(value: i32) -> Self {
		Self(value)
	}
}
impl From<i16> for VarInt {
	fn from(value: i16) -> Self {
		Self(value as i32)
	}
}
impl From<u16> for VarInt {
	fn from(value: u16) -> Self {
		Self(value as i32)
	}
}
impl From<i8> for VarInt {
	fn from(value: i8) -> Self {
		Self(value as i32)
	}
}
impl From<u8> for VarInt {
	fn from(value: u8) -> Self {
		Self(value as i32)
	}
}

impl TryFrom<usize> for VarInt {
	type Error = std::num::TryFromIntError;

	fn try_from(value: usize) -> std::result::Result<Self, Self::Error> {
		Ok(VarInt(value.try_into()?))
	}
}
impl TryFrom<VarInt> for usize {
	type Error = std::num::TryFromIntError;

	fn try_from(value: VarInt) -> std::result::Result<Self, Self::Error> {
		value.0.try_into()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const TEST_CASES: &[(i32, &[u8])] = &[
		(0, &[0x00]),
		(1, &[0x01]),
		(2, &[0x02]),
		(127, &[0x7F]),
		(128, &[0x80, 0x01]),
		(255, &[0xFF, 0x01]),
		(25565, &[0xDD, 0xC7, 0x01]),
		(2097151, &[0xFF, 0xFF, 0x7F]),
		(2147483647, &[0xFF, 0xFF, 0xFF, 0xFF, 0x07]),
		(-1, &[0xFF, 0xFF, 0xFF, 0xFF, 0x0F]),
		(-2147483648, &[0x80, 0x80, 0x80, 0x80, 0x08]),
	];

	#[test]
	fn varint_read() {
		for (i, case) in TEST_CASES.into_iter().enumerate() {
			let result = VarInt::mcp_read(&mut &case.1[..]).unwrap();
			assert_eq!(result.0, case.0, "{i}");
		}
	}

	#[test]
	fn varint_write() {
		for (i, case) in TEST_CASES.into_iter().enumerate() {
			let mut buf = Vec::new();
			let result = VarInt(case.0).mcp_write(&mut buf);
			assert_eq!(result, case.1.len(), "{i}");
			assert_eq!(buf, case.1, "{i}");
		}
	}

	#[test]
	fn varint_len() {
		for (i, case) in TEST_CASES.into_iter().enumerate() {
			assert_eq!(VarInt(case.0).num_bytes(), case.1.len(), "{i}");
		}
	}
}
