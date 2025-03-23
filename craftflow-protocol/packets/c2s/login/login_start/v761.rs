// [
//     "container",
//     [
//         {
//             "name": "username",
//             "type": "string"
//         },
//         {
//             "name": "playerUUID",
//             "type": [
//                 "option",
//                 "UUID"
//             ]
//         }
//     ]
// ]

mcp! {
	#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
	pub struct LoginStartV761 {
		pub username: (String),
		pub player_uuid: (Option<(u128)>),
	}
}
