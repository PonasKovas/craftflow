// [
//     "container",
//     [
//         {
//             "name": "reason",
//             "type": "string"
//         }
//     ]
// ]

mcp! {
	#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
	pub struct DisconnectV5 {
		pub reason: String,
	}
}
