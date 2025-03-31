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
//             "name": "dimension",
//             "type": "i32"
//         },
//         {
//             "name": "maxPlayers",
//             "type": "u8"
//         },
//         {
//             "name": "levelType",
//             "type": "string"
//         },
//         {
//             "name": "viewDistance",
//             "type": "varint"
//         },
//         {
//             "name": "reducedDebugInfo",
//             "type": "bool"
//         }
//     ]
// ]

mcp! {
	#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
	pub struct LoginV477 {
		pub entity_id: (i32),
		pub game_mode: (u8),
		pub dimension: (i32),
		pub max_players: (u8),
		pub level_type: (String),
		pub view_distance: (VarInt),
		pub reduced_debug_info: (bool),
	}
}
