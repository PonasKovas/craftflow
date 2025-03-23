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
	pub struct DisconnectV764 {
		pub reason: String,
	}
}
