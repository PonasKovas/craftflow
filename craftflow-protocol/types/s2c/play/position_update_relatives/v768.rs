// [
//     "bitflags",
//     {
//         "type": "u32",
//         "flags": [
//             "x",
//             "y",
//             "z",
//             "yaw",
//             "pitch",
//             "dx",
//             "dy",
//             "dz",
//             "yawDelta"
//         ]
//     }
// ]

#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
pub struct PositionUpdateRelativesV768 {
	pub x: bool,
	pub y: bool,
	pub z: bool,
	pub yaw: bool,
	pub pitch: bool,
	pub dx: bool,
	pub dy: bool,
	pub dz: bool,
	pub yaw_delta: bool,
}

impl MCP for PositionUpdateRelativesV768 {
	type Data = Self;
}

impl MCPWrite for PositionUpdateRelativesV768 {
	fn mcp_write(data: &Self, output: &mut Vec<u8>) -> usize {
		let mut flags = 0u32;
		if data.x {
			flags |= 1 << 0;
		}
		if data.y {
			flags |= 1 << 1;
		}
		if data.z {
			flags |= 1 << 2;
		}
		if data.yaw {
			flags |= 1 << 3;
		}
		if data.pitch {
			flags |= 1 << 4;
		}
		if data.dx {
			flags |= 1 << 5;
		}
		if data.dy {
			flags |= 1 << 6;
		}
		if data.dz {
			flags |= 1 << 7;
		}
		if data.yaw_delta {
			flags |= 1 << 8;
		}
		u32::mcp_write(&flags, output)
	}
}

impl<'a> MCPRead<'a> for PositionUpdateRelativesV768 {
	fn mcp_read(input: &mut &'a [u8]) -> Result<Self> {
		let flags = u32::mcp_read(input)?;
		Ok(Self {
			x: flags & (1 << 0) != 0,
			y: flags & (1 << 1) != 0,
			z: flags & (1 << 2) != 0,
			yaw: flags & (1 << 3) != 0,
			pitch: flags & (1 << 4) != 0,
			dx: flags & (1 << 5) != 0,
			dy: flags & (1 << 6) != 0,
			dz: flags & (1 << 7) != 0,
			yaw_delta: flags & (1 << 8) != 0,
		})
	}
}
