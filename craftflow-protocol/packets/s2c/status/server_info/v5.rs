// [
//     "container",
//     [
//         {
//             "name": "response",
//             "type": "string"
//         }
//     ]
// ]

mcp! {
	#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
	pub struct ServerInfoV5 {
		pub response: String,
	}
}
