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
//             "name": "doLimitedCrafting",
//             "type": "bool"
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
//             "name": "gameMode",
//             "type": "u8"
//         },
//         {
//             "name": "previousGameMode",
//             "type": "i8"
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
//         }
//     ]
// ]

mcp! {
	#[derive(Debug, PartialEq, Clone)]
	pub struct LoginV764 {
		pub entity_id: (i32),
		pub is_hardcore: (bool),
		pub world_names: (Array<(String)>),
		pub max_players: (VarInt),
		pub view_distance: (VarInt),
		pub simulation_distance: (VarInt),
		pub reduced_debug_info: (bool),
		pub enable_respawn_screen: (bool),
		pub do_limited_crafting: (bool),
		pub world_type: (String),
		pub world_name: (String),
		pub hashed_seed: (i64),
		pub game_mode: (u8),
		pub previous_game_mode: (i8),
		pub is_debug: (bool),
		pub is_flat: (bool),
		pub death: (Option<(DeathInfo)>),
		pub portal_cooldown: (VarInt),
	}
}

mcp! {
	#[derive(Debug, PartialEq, Clone)]
	pub struct DeathInfo {
		pub dimension_name: (String),
		pub location: (PositionV477),
	}
}
