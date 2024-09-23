use crate::{MCPBaseRead, MCPBaseWrite};
use std::io::Write;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
	pub x: i32,
	pub y: i32,
	pub z: i32,
}

impl MCPBaseWrite for Position {
	fn write(&self, protocol_version: u32, output: &mut impl Write) -> crate::Result<usize> {
		let packed = (((self.x as i64) & 0x3FFFFFF) << 38)
			| (((self.z as i64) & 0x3FFFFFF) << 12)
			| ((self.y as i64) & 0xFFF);

		packed.write(protocol_version, output)
	}
}

impl MCPBaseRead for Position {
	fn read(protocol_version: u32, input: &[u8]) -> crate::Result<(&[u8], Self)> {
		let (input, packed) = i64::read(protocol_version, input)?;

		let x = (packed >> 38) as i32;
		let y = ((packed << 52) >> 52) as i32;
		let z = ((packed << 26) >> 38) as i32;

		Ok((input, Position { x, y, z }))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_position() {
		let position = Position {
			x: 0,
			y: -1345,
			z: 98765,
		};

		let mut buffer = Vec::new();
		position.write(0, &mut buffer).unwrap();

		let (rest, read_position) = Position::read(0, &buffer).unwrap();
		assert_eq!(rest.len(), 0);
		assert_eq!(position, read_position);
	}
}
