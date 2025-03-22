// [
//     "container",
//     [
//         {
//             "name": "username",
//             "type": "string"
//         }
//     ]
// ]

mcp! {
	#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
	pub struct LoginStartV5 {
		pub username: String,
	}
}
