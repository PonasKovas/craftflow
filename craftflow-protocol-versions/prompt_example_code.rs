#[derive(Debug, PartialEq, Clone)]
pub struct EntityInformation<'a> {
	pub entity_id: i32,
	pub entity_type: VarInt,
	pub entity_num: VarLong,
	pub blob: Buffer<'a, u64>,
	pub entity_uuid: u128,
	pub is_player: Option<Cow<'a, str>>,
	pub position: Position,
	pub information: Information<'a>,
	pub associated_data: Nbt,
	pub block_nbt: AnonymousNbt,
	pub history: Array<'a, VarInt, VarInt>,
}

#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Eq, Ord)]
pub struct Position {
	pub x: i32,
	pub z: i32,
	pub y: i16,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Information<'a> {
	pub inventory: Array<'a, u8, VarInt>,
	pub priority: f32,
	pub world_status: WorldStatus<'a>,
	pub plugin_data: RestBuffer<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum WorldStatus<'a> {
	V1(Cow<'a, str>),
	V2 { velocity: f64, jumped: bool },
	Default,
}

impl<'a> MCPWrite for EntityInformation<'a> {
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
		written_bytes += self.associated_data.write(output)?;
		written_bytes += self.block_nbt.write(output)?;
		written_bytes += self.history.write(output)?;

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
impl<'a> MCPWrite for Information<'a> {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		let mut written_bytes = 0;
		written_bytes += self.inventory.write(output)?;
		written_bytes += self.priority.write(output)?;
		written_bytes += self.world_status.write(output)?;
		written_bytes += self.plugin_data.write(output)?;

		Ok(written_bytes)
	}
}
impl<'a> MCPWrite for WorldStatus<'a> {
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

impl<'a> MCPRead<'a> for EntityInformation<'a> {
	fn read(input: &[u8]) -> Result<(&[u8], Self)> {
		let (input, entity_id) = i32::read(input)?;
		let (input, entity_type) = VarInt::read(input)?;
		let (input, entity_num) = VarLong::read(input)?;
		let (input, blob) = Buffer::read(input)?;
		let (input, entity_uuid) = u128::read(input)?;
		let (input, is_player) = Option::read(input)?;
		let (input, position) = Position::read(input)?;
		let (input, information) = Information::read(input)?;
		let (input, associated_data) = Nbt::read(input)?;
		let (input, block_nbt) = AnonymousNbt::read(input)?;
		let (input, history) = Array::read(input)?;

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
				associated_data,
				block_nbt,
				history,
			},
		))
	}
}
impl<'a> MCPRead<'a> for Position {
	fn read(input: &[u8]) -> Result<(&[u8], Self)> {
		let (input, position_bitfield) = i64::read(input)?;

		let x = (position_bitfield >> 38) as i32 & 0x3FFFFFF;
		let z = (position_bitfield >> 12) as i32 & 0x3FFFFFF;
		let y = (position_bitfield >> 0) as i16 & 0xFFF;

		Ok((input, Self { x, z, y }))
	}
}
impl<'a> MCPRead<'a> for Information<'a> {
	fn read(input: &[u8]) -> Result<(&[u8], Self)> {
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
impl<'a> MCPRead<'a> for WorldStatus<'a> {
	fn read(input: &[u8]) -> Result<(&[u8], Self)> {
		let (input, variant) = VarInt::read(input)?;

		match variant.0 {
			0 => {
				let (input, s) = Cow::read(input)?;
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
