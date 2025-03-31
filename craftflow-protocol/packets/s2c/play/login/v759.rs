// [
//     "container",
//     [
//         {
//             "name": "entityId",
//             "type": "i32"
//         },
//         {
//             "name": "isHardcore",
//             "type": "bool"
//         },
//         {
//             "name": "gameMode",
//             "type": "u8"
//         },
//         {
//             "name": "previousGameMode",
//             "type": "i8"
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
//             "name": "worldType",
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
//             "type": "varint"
//         },
//         {
//             "name": "viewDistance",
//             "type": "varint"
//         },
//         {
//             "name": "simulationDistance",
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
//         }
//     ]
// ]

mcp! {
	#[derive(Debug, PartialEq, Clone)]
	pub struct LoginV759 {
		pub entity_id: (i32),
		pub is_hardcore: (bool),
		pub game_mode: (u8),
		pub previous_game_mode: (i8),
		pub world_names: (Array<(String)>),
		pub dimension_codec: (NamedNbt),
		pub world_type: (String),
		pub world_name: (String),
		pub hashed_seed: (i64),
		pub max_players: (VarInt),
		pub view_distance: (VarInt),
		pub simulation_distance: (VarInt),
		pub reduced_debug_info: (bool),
		pub enable_respawn_screen: (bool),
		pub is_debug: (bool),
		pub is_flat: (bool),
		pub death: (Option<(DeathInfo)>),
	}
}

mcp! {
	#[derive(Debug, PartialEq, Clone)]
	pub struct DeathInfo {
		pub dimension_name: (String),
		pub location: (PositionV477),
	}
}
