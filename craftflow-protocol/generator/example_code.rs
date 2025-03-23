mcp! {
	#[derive(Debug, PartialEq, Clone)]
	pub struct EntityInformationV18 {
		pub entity_id: (i32),
		pub entity_type: (OptVarInt),
		pub entity_num: (VarLong),
		pub blob: (Buffer<(u64)>),
		pub entity_uuid: (u128),
		pub is_player: (Option<(String)>),
		pub position: (Position),
		pub information: (Information),
		pub associated_data: (NamedNbt),
		pub block_nbt: (Nbt),
		pub history: (Array<(VarInt)>),
		pub crypto: (Crypto),
	}
}

mcp! {
	#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Eq, Ord)]
	pub struct Position {
		pub x: (i32),
		pub z: (i32),
		pub y: (i16),
	}
}

mcp! {
	#[derive(Debug, PartialEq, Clone)]
	pub struct Information {
		pub inventory: (Array<(u8)>),
		pub priority: (f32),
		pub world_status: (WorldStatus),
		pub plugin_data: (RestBuffer),
	}
}

#[derive(Debug, PartialEq, Clone)]
pub enum WorldStatus {
	V1(<String as MCP>::Data),
	V2 {
		velocity: <f64 as MCP>::Data,
		jumped: <bool as MCP>::Data,
	},
}

impl MCP for WorldStatus {
	type Data = Self;
}
impl MCPWrite for WorldStatus {
	fn mcp_write(data: &Self, output: &mut Vec<u8>) -> usize {
		let mut written_bytes = 0;

		match data {
			WorldStatus::V1(s) => {
				written_bytes += VarInt::mcp_write(&0, output);
				written_bytes += String::mcp_write(s, output);
			}
			WorldStatus::V2 { velocity, jumped } => {
				written_bytes += VarInt::mcp_write(&1, output);
				written_bytes += f64::mcp_write(velocity, output);
				written_bytes += bool::mcp_write(jumped, output);
			}
		}

		written_bytes
	}
}

impl<'a> MCPRead<'a> for WorldStatus {
	fn mcp_read(input: &mut &'a [u8]) -> Result<Self> {
		let variant = VarInt::mcp_read(input)?;

		match variant {
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
		verify_token: <Buffer as MCP>::Data,
	},
	WithoutVerifyToken {
		salt: <i64 as MCP>::Data,
		message_signature: <Buffer as MCP>::Data,
	},
}

impl MCP for Crypto {
	type Data = Self;
}
impl MCPWrite for Crypto {
	fn mcp_write(data: &Self, output: &mut Vec<u8>) -> usize {
		let mut written_bytes = 0;

		match data {
			Crypto::WithVerifyToken { verify_token } => {
				written_bytes += bool::mcp_write(&true, output);
				written_bytes += <Buffer>::mcp_write(verify_token, output);
			}
			Crypto::WithoutVerifyToken {
				salt,
				message_signature,
			} => {
				written_bytes += bool::mcp_write(&false, output);
				written_bytes += i64::mcp_write(salt, output);
				written_bytes += <Buffer>::mcp_write(message_signature, output);
			}
		}

		written_bytes
	}
}

impl<'a> MCPRead<'a> for Crypto {
	fn mcp_read(input: &mut &'a [u8]) -> Result<Self> {
		let has_verify_token = bool::mcp_read(input)?;

		if has_verify_token {
			let verify_token = <Buffer>::mcp_read(input)?;
			Ok(Self::WithVerifyToken { verify_token })
		} else {
			let salt = i64::mcp_read(input)?;
			let message_signature = <Buffer>::mcp_read(input)?;
			Ok(Self::WithoutVerifyToken {
				salt,
				message_signature,
			})
		}
	}
}
