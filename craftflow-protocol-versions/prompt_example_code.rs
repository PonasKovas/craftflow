#[derive(Debug, PartialEq, Clone)]
pub struct EntityInformation {
	pub entity_id: i32,
	pub entity_type: VarInt,
	pub entity_num: VarLong,
	pub blob: Buffer<u64>,
	pub entity_uuid: u128,
	pub is_player: Option<String>,
	pub position: Position,
	pub information: Information,
	pub extra_data: TopBitSetArray<i32>,
	pub associated_data: Nbt,
	pub block_nbt: AnonymousNbt,
}

#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Eq, Ord)]
pub struct Position {
	pub x: i32,
	pub z: i32,
	pub y: i16,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Information {
	pub inventory: Array<u8, VarInt>,
	pub priority: f32,
	pub world_status: WorldStatus,
	pub plugin_data: RestBuffer,
}

#[derive(Debug, PartialEq, Clone)]
pub enum WorldStatus {
	V1(String),
	V2 { velocity: f64, jumped: bool },
	Default,
}

impl MCPWrite for EntityInformation {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		let mut written_bytes = 0;

		written_bytes += self.entity_id.write(output)?;
		written_bytes += self.entity_type.write(output)?;
		written_bytes += self.entity_num.write(output)?;
		written_bytes += self.blob.write(output)?;
		written_bytes += self.entity_uuid.write(output)?;
		written_bytes += self.is_player.write(output)?;
		written_bytes += self.position.write(output)?;
		written_bytes += self.information.write(output)?;
		written_bytes += self.extra_data.write(output)?;
		written_bytes += self.associated_data.write(output)?;
		written_bytes += self.block_nbt.write(output)?;

		Ok(written_bytes)
	}
}
impl MCPWrite for Position {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		let mut position_bitfield = 0i64;
		position_bitfield |= (self.x as i64 & 0x3FFFFFF) << 38;
		position_bitfield |= (self.z as i64 & 0x3FFFFFF) << 12;
		position_bitfield |= (self.y as i64 & 0xFFF) << 0;

		position_bitfield.write(output)
	}
}
impl MCPWrite for Information {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		let mut written_bytes = 0;
		written_bytes += self.inventory.write(output)?;
		written_bytes += self.priority.write(output)?;
		written_bytes += self.world_status.write(output)?;
		written_bytes += self.plugin_data.write(output)?;

		Ok(written_bytes)
	}
}
impl MCPWrite for WorldStatus {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		let mut written_bytes = 0;

		match self {
			WorldStatus::V1(s) => {
				written_bytes += VarInt(0).write(output)?;
				written_bytes += s.write(output)?;
			}
			WorldStatus::V2 { velocity, jumped } => {
				written_bytes += VarInt(1).write(output)?;
				written_bytes += velocity.write(output)?;
				written_bytes += jumped.write(output)?;
			}
			WorldStatus::Default => {
				// Any non-variant, in this case we choose 2
				written_bytes += VarInt(2).write(output)?;
			}
		}

		Ok(written_bytes)
	}
}

impl MCPRead for EntityInformation {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input, entity_id) = i32::read(input)?;
		let (input, entity_type) = VarInt::read(input)?;
		let (input, entity_num) = VarLong::read(input)?;
		let (input, blob) = Buffer::read(input)?;
		let (input, entity_uuid) = u128::read(input)?;
		let (input, is_player) = Option::<String>::read(input)?;
		let (input, position) = Position::read(input)?;
		let (input, information) = Information::read(input)?;
		let (input, extra_data) = TopBitSetArray::<i32>::read(input)?;
		let (input, associated_data) = Nbt::read(input)?;
		let (input, block_nbt) = AnonymousNbt::read(input)?;

		Ok((
			input,
			Self {
				entity_id,
				entity_type,
				entity_num,
				blob,
				entity_uuid,
				is_player,
				position,
				information,
				extra_data,
				associated_data,
				block_nbt,
			},
		))
	}
}
impl MCPRead for Position {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input, position_bitfield) = i64::read(input)?;

		let x = (position_bitfield >> 38) as i32 & 0x3FFFFFF;
		let z = (position_bitfield >> 12) as i32 & 0x3FFFFFF;
		let y = (position_bitfield >> 0) as i16 & 0xFFF;

		Ok((input, Self { x, z, y }))
	}
}
impl MCPRead for Information {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input, inventory) = Array::<u8, VarInt>::read(input)?;
		let (input, priority) = f32::read(input)?;
		let (input, world_status) = WorldStatus::read(input)?;
		let (input, plugin_data) = RestBuffer::read(input)?;

		Ok((
			input,
			Self {
				inventory,
				priority,
				world_status,
				plugin_data,
			},
		))
	}
}
impl MCPRead for WorldStatus {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input, variant) = VarInt::read(input)?;

		match variant.0 {
			0 => {
				let (input, s) = String::read(input)?;
				Ok((input, Self::V1(s)))
			}
			1 => {
				let (input, velocity) = f64::read(input)?;
				let (input, jumped) = bool::read(input)?;
				Ok((input, Self::V2 { velocity, jumped }))
			}
			_ => Ok((input, Self::Default)),
		}
	}
}
