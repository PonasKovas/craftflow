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
//             "name": "worldState",
//             "type": "SpawnInfo"
//         },
//         {
//             "name": "enforcesSecureChat",
//             "type": "bool"
//         }
//     ]
// ]

mcp! {
	#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
	pub struct LoginV766 {
		pub entity_id: (i32),
		pub is_hardcore: (bool),
		pub world_names: (Array<(String)>),
		pub max_players: (VarInt),
		pub view_distance: (VarInt),
		pub simulation_distance: (VarInt),
		pub reduced_debug_info: (bool),
		pub enable_respawn_screen: (bool),
		pub do_limited_crafting: (bool),
		// pub world_state: (SpawnInfo),
		pub enforces_secure_chat: (bool),
	}
}
