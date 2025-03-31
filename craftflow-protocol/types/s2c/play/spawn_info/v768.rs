// [
//     "container",
//     [
//         {
//             "name": "dimension",
//             "type": "varint"
//         },
//         {
//             "name": "name",
//             "type": "string"
//         },
//         {
//             "name": "hashedSeed",
//             "type": "i64"
//         },
//         {
//             "name": "gamemode",
//             "type": [
//                 "mapper",
//                 {
//                     "type": "i8",
//                     "mappings": {
//                         "0": "survival",
//                         "1": "creative",
//                         "2": "adventure",
//                         "3": "spectator"
//                     }
//                 }
//             ]
//         },
//         {
//             "name": "previousGamemode",
//             "type": "u8"
//         },
//         {
//             "name": "isDebug",
//             "type": "bool"
//         },
//         {
//             "name": "isFlat",
//             "type": "bool"
//         },
//         {
//             "name": "death",
//             "type": [
//                 "option",
//                 [
//                     "container",
//                     [
//                         {
//                             "name": "dimensionName",
//                             "type": "string"
//                         },
//                         {
//                             "name": "location",
//                             "type": "position"
//                         }
//                     ]
//                 ]
//             ]
//         },
//         {
//             "name": "portalCooldown",
//             "type": "varint"
//         },
//         {
//             "name": "seaLevel",
//             "type": "varint"
//         }
//     ]
// ]

mcp! {
	#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
	pub struct SpawnInfoV768 {
		pub dimension: (VarInt),
		pub name: (String),
		pub hashed_seed: (i64),
		pub gamemode: (GameMode),
		pub previous_gamemode: (u8),
		pub is_debug: (bool),
		pub is_flat: (bool),
		pub death: (Option<(DeathInfo)>),
		pub portal_cooldown: (VarInt),
		pub sea_level: (VarInt),
	}
}

#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Eq, Ord)]
pub enum GameMode {
	Survival,
	Creative,
	Adventure,
	Spectator,
}

impl MCP for GameMode {
	type Data = Self;
}

impl MCPWrite for GameMode {
	fn mcp_write(data: &Self, output: &mut Vec<u8>) -> usize {
		let value = match data {
			GameMode::Survival => 0,
			GameMode::Creative => 1,
			GameMode::Adventure => 2,
			GameMode::Spectator => 3,
		};
		i8::mcp_write(&value, output)
	}
}

impl<'a> MCPRead<'a> for GameMode {
	fn mcp_read(input: &mut &'a [u8]) -> Result<Self> {
		let value = i8::mcp_read(input)?;
		match value {
			0 => Ok(GameMode::Survival),
			1 => Ok(GameMode::Creative),
			2 => Ok(GameMode::Adventure),
			3 => Ok(GameMode::Spectator),
			_ => Err(Error::InvalidEnumTag {
				tag: value as i64,
				enum_name: "GameMode",
			}),
		}
	}
}

mcp! {
	#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
	pub struct DeathInfo {
		pub dimension_name: (String),
		pub location: (PositionV477),
	}
}
