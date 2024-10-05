#[derive(Debug, PartialEq, Clone, Hash, PartialOrd)]
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

#[derive(Debug, PartialEq, Clone, Hash, PartialOrd)]
pub struct Information {
	pub inventory: Array<u8, VarInt>,
	pub priority: f32,
	pub world_status: WorldStatus,
	pub plugin_data: RestBuffer,
}

#[derive(Debug, PartialEq, Clone, Hash, PartialOrd)]
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

		let mut position_bitfield = 0i64;
		position_bitfield |= (self.position.x & 0x3FFFFFF) << 38;
		position_bitfield |= (self.position.z & 0x3FFFFFF) << 12;
		position_bitfield |= (self.position.y & 0xFFF) << 0;
		written_bytes += position_bitfield.write(output)?;

		written_bytes += self.information.inventory.write(output)?;
		written_bytes += self.information.priority.write(output)?;
		match &self.information.world_status {
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
		written_bytes += self.information.plugin_data.write(output)?;

		written_bytes += self.extra_data.write(output)?;
		written_bytes += self.associated_data.write(output)?;
		written_bytes += self.block_nbt.write(output)?;

		Ok(written_bytes)
	}
}
impl MCPRead for EntityInformation {
	fn read(input: &[u8]) -> Result<(&[u8], Self)> {
		let (input, entity_id) = i32::read(input)?;
		let (mut input, properties_len) = i32::read(input)?;
		let mut properties = Vec::new();
		for _ in 0..properties_len {
			let (i, key) = String::read(input)?;
			let (i, value) = f64::read(input)?;
			let (i, modifiers_len) = i16::read(input)?;
			let mut modifiers = Vec::new();
			for _ in 0..modifiers_len {
				let (ii, uuid) = u128::read(input)?;
				let (ii, amount) = f64::read(input)?;
				let (ii, operation) = i8::read(input)?;
				modifiers.push(PropertyModifier {
					uuid,
					amount,
					operation,
				});
				i = ii;
			}
			properties.push(Property {
				key,
				value,
				modifiers,
			});
			input = i;
		}

		Ok((
			input,
			Self {
				entity_id,
				properties,
			},
		))
	}
}
