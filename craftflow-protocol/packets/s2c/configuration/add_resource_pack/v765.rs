// [
//     "container",
//     [
//         {
//             "name": "uuid",
//             "type": "UUID"
//         },
//         {
//             "name": "url",
//             "type": "string"
//         },
//         {
//             "name": "hash",
//             "type": "string"
//         },
//         {
//             "name": "forced",
//             "type": "bool"
//         },
//         {
//             "name": "promptMessage",
//             "type": [
//                 "option",
//                 "anonymousNbt"
//             ]
//         }
//     ]
// ]

mcp! {
	#[derive(Debug, PartialEq, Clone)]
	pub struct AddResourcePackV765 {
		pub uuid: (u128),
		pub url: (String),
		pub hash: (String),
		pub forced: (bool),
		pub prompt_message: (Option<(Nbt)>),
	}
}
