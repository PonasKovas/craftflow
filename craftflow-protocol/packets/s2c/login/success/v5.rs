// [
//     "container",
//     [
//         {
//             "name": "uuid",
//             "type": "string"
//         },
//         {
//             "name": "username",
//             "type": "string"
//         }
//     ]
// ]

mcp! {
	#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
	pub struct SuccessV5 {
		pub uuid: String,
		pub username: String,
	}
}
