// [
//     "container",
//     [
//         {
//             "name": "username",
//             "type": "string"
//         },
//         {
//             "name": "playerUUID",
//             "type": "UUID"
//         }
//     ]
// ]

mcp! {
	#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
	pub struct LoginStartV764 {
		pub username: String,
		pub player_uuid: u128,
	}
}
