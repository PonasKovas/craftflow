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
//             "type": "i8"
//         },
//         {
//             "name": "difficulty",
//             "type": "u8"
//         },
//         {
//             "name": "maxPlayers",
//             "type": "u8"
//         },
//         {
//             "name": "levelType",
//             "type": "string"
//         }
//     ]
// ]

mcp! {
	#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
	pub struct LoginV5 {
		pub entity_id: (i32),
		pub game_mode: (u8),
		pub dimension: (i8),
		pub difficulty: (u8),
		pub max_players: (u8),
		pub level_type: (String),
	}
}
