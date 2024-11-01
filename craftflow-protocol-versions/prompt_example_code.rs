define_type! {
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
}

define_type! {
	#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Eq, Ord)]
	pub struct Position {
		pub x: i32,
		pub z: i32,
		pub y: i16,
	}
}

define_type! {
	#[derive(Debug, PartialEq, Clone)]
	pub struct Information<'a> {
		pub inventory: Array<'a, u8, VarInt>,
		pub priority: f32,
		pub world_status: WorldStatus<'a>,
		pub plugin_data: RestBuffer<'a>,
	}
}

#[derive(Debug, PartialEq, Clone)]
pub enum WorldStatus<'a> {
	V1(Cow<'a, str>),
	V2 { velocity: f64, jumped: bool },
	Default,
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

impl<'a> MCPRead<'a> for WorldStatus<'a> {
	fn read(input: &'a [u8]) -> Result<(&'a [u8], Self)> {
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
