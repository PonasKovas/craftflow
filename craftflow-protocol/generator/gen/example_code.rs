mcp! {
	#[derive(Debug, PartialEq, Clone)]
	pub struct EntityInformationV18 {
		pub entity_id: i32,
		pub entity_type: VarInt,
		pub entity_num: VarLong,
		pub blob: Buffer<1_000_000, u64>,
		pub entity_uuid: u128,
		pub is_player: Option<String>,
		pub position: Position,
		pub information: Information,
		pub associated_data: NamedNbt,
		pub block_nbt: Nbt,
		pub history: Array<VarInt, 1_000_000, VarInt>,
		pub crypto: Crypto,
	}
}

mcp! {
	#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Eq, Ord)]
	pub struct Position {
		pub x: i32,
		pub z: i32,
		pub y: i16,
	}
}

mcp! {
	#[derive(Debug, PartialEq, Clone)]
	pub struct Information {
		pub inventory: Array<u8, 1_000_000, VarInt>,
		pub priority: f32,
		pub world_status: WorldStatus,
		pub plugin_data: RestBuffer,
	}
}

#[derive(Debug, PartialEq, Clone)]
pub enum WorldStatus {
	V1(String),
	V2 { velocity: f64, jumped: bool },
}

impl MCPWrite for WorldStatus {
	fn mcp_write(&self, output: &mut Vec<u8>) -> usize {
		let mut written_bytes = 0;

		match self {
			WorldStatus::V1(s) => {
				written_bytes += VarInt(0).mcp_write(output);
				written_bytes += s.mcp_write(output);
			}
			WorldStatus::V2 { velocity, jumped } => {
				written_bytes += VarInt(1).mcp_write(output);
				written_bytes += velocity.mcp_write(output);
				written_bytes += jumped.mcp_write(output);
			}
		}

		written_bytes
	}
}

impl<'a> MCPRead<'a> for WorldStatus {
	fn mcp_read(input: &mut &'a [u8]) -> Result<Self> {
		let variant = VarInt::mcp_read(input)?;

		match variant.0 {
			0 => {
				let s = String::mcp_read(input)?;
				Ok(Self::V1(s))
			}
			1 => {
				let velocity = f64::mcp_read(input)?;
				let jumped = bool::mcp_read(input)?;
				Ok(Self::V2 { velocity, jumped })
			}
			other => Err(Error::InvalidEnumTag {
				tag: other as i64,
				enum_name: "WorldStatus",
			}),
		}
	}
}

#[derive(Debug, PartialEq, Clone)]
pub enum Crypto {
	WithVerifyToken {
		verify_token: Buffer,
	},
	WithoutVerifyToken {
		salt: i64,
		message_signature: Buffer,
	},
}

impl MCPWrite for Crypto {
	fn mcp_write(&self, output: &mut Vec<u8>) -> usize {
		let mut written_bytes = 0;

		match self {
			Crypto::WithVerifyToken { verify_token } => {
				written_bytes += true.mcp_write(output);
				written_bytes += verify_token.mcp_write(output);
			}
			Crypto::WithoutVerifyToken {
				salt,
				message_signature,
			} => {
				written_bytes += false.mcp_write(output);
				written_bytes += salt.mcp_write(output);
				written_bytes += message_signature.mcp_write(output);
			}
		}

		written_bytes
	}
}

impl<'a> MCPRead<'a> for Crypto {
	fn mcp_read(input: &mut &'a [u8]) -> Result<Self> {
		let has_verify_token = bool::mcp_read(input)?;

		if has_verify_token {
			let verify_token = Buffer::mcp_read(input)?;
			Ok(Self::WithVerifyToken { verify_token })
		} else {
			let salt = i64::mcp_read(input)?;
			let message_signature = Buffer::mcp_read(input)?;
			Ok(Self::WithoutVerifyToken {
				salt,
				message_signature,
			})
		}
	}
}
