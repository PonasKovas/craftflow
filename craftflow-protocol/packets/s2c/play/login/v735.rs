// [
//     "container",
//     [
//         {
//             "name": "entityId",
//             "type": "i32"
//         },
//         {
//             "name": "gameMode",
//             "type": "u8"
//         },
//         {
//             "name": "previousGameMode",
//             "type": "u8"
//         },
//         {
//             "name": "worldNames",
//             "type": [
//                 "array",
//                 {
//                     "countType": "varint",
//                     "type": "string"
//                 }
//             ]
//         },
//         {
//             "name": "dimensionCodec",
//             "type": "nbt"
//         },
//         {
//             "name": "dimension",
//             "type": "string"
//         },
//         {
//             "name": "worldName",
//             "type": "string"
//         },
//         {
//             "name": "hashedSeed",
//             "type": "i64"
//         },
//         {
//             "name": "maxPlayers",
//             "type": "u8"
//         },
//         {
//             "name": "viewDistance",
//             "type": "varint"
//         },
//         {
//             "name": "reducedDebugInfo",
//             "type": "bool"
//         },
//         {
//             "name": "enableRespawnScreen",
//             "type": "bool"
//         },
//         {
//             "name": "isDebug",
//             "type": "bool"
//         },
//         {
//             "name": "isFlat",
//             "type": "bool"
//         }
//     ]
// ]

mcp! {
	#[derive(Debug, PartialEq, Clone)]
	pub struct LoginV735 {
		pub entity_id: (i32),
		pub game_mode: (u8),
		pub previous_game_mode: (u8),
		pub world_names: (Array<(String)>),
		pub dimension_codec: (NamedNbt),
		pub dimension: (String),
		pub world_name: (String),
		pub hashed_seed: (i64),
		pub max_players: (u8),
		pub view_distance: (VarInt),
		pub reduced_debug_info: (bool),
		pub enable_respawn_screen: (bool),
		pub is_debug: (bool),
		pub is_flat: (bool),
	}
}
