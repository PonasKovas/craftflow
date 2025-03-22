use crate::{Error, Result};

use super::{MCP, MCPRead, MCPWrite};

/// A Minecraft Protocol VarLong
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct VarLong;

// impl VarLong {
// 	/// Returns the length (in bytes) of the VarInt in the Minecraft Protocol format.
// 	pub fn num_bytes(data: i64) -> usize {
// 		let value = data as u64;
// 		if value == 0 {
// 			return 1;
// 		}
// 		let bits_needed = 64 - value.leading_zeros();
// 		((bits_needed + 6) / 7) as usize
// 	}
// }

impl MCP for VarLong {
	type Data = i64;
}
impl<'a> MCPRead<'a> for VarLong {
	fn mcp_read(input: &mut &[u8]) -> Result<i64> {
		let mut num_read = 0; // Count of bytes that have been read
		let mut result = 0i64; // The VarInt being constructed

		loop {
			// VarInts are at most 5 bytes long.
			if num_read == 10 {
				return Err(Error::VarLongTooBig);
			}

			// Read a byte
			let byte = u8::mcp_read(input)?;

			// Extract the 7 lower bits (the data bits) and cast to i32
			let value = (byte & 0b0111_1111) as i64;

			// Shift the data bits to the correct position and add them to the result
			result |= value << (7 * num_read);

			num_read += 1;

			// If the high bit is not set, this was the last byte in the VarInt
			if (byte & 0b1000_0000) == 0 {
				break;
			}
		}

		Ok(result)
	}
}

impl MCPWrite for VarLong {
	fn mcp_write(data: &i64, output: &mut Vec<u8>) -> usize {
		let mut i = 0;
		let mut value = *data;

		loop {
			// Take the 7 lower bits of the value
			let mut temp = (value & 0b0111_1111) as u8;

			// Shift the value 7 bits to the right.
			value = ((value as u64) >> 7) as i64;

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

#[cfg(test)]
mod tests {
	use super::*;

	const TEST_CASES: &[(i64, &[u8])] = &[
		(0, &[0x00]),
		(1, &[0x01]),
		(2, &[0x02]),
		(127, &[0x7F]),
		(128, &[0x80, 0x01]),
		(255, &[0xFF, 0x01]),
		(25565, &[0xDD, 0xC7, 0x01]),
		(2097151, &[0xFF, 0xFF, 0x7F]),
		(2147483647, &[0xFF, 0xFF, 0xFF, 0xFF, 0x07]),
		(
			9223372036854775807,
			&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F],
		),
		(
			-1,
			&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01],
		),
		(
			-9223372036854775808,
			&[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01],
		),
	];

	#[test]
	fn varlong_read() {
		for (i, case) in TEST_CASES.into_iter().enumerate() {
			let result = VarLong::mcp_read(&mut &case.1[..]).unwrap();
			assert_eq!(result, case.0, "{i}");
		}
	}

	#[test]
	fn varlong_write() {
		for (i, case) in TEST_CASES.into_iter().enumerate() {
			let mut buf = Vec::new();
			let result = VarLong::mcp_write(&case.0, &mut buf);
			assert_eq!(result, case.1.len(), "{i}");
			assert_eq!(buf, case.1, "{i}");
		}
	}

	// #[test]
	// fn varlong_len() {
	// 	for (i, case) in TEST_CASES.into_iter().enumerate() {
	// 		assert_eq!(VarLong::num_bytes(case.0), case.1.len(), "{i}");
	// 	}
	// }
}
