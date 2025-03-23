// [
//     "container",
//     [
//         {
//             "name": "reason",
//             "type": "anonymousNbt"
//         }
//     ]
// ]

mcp! {
	#[derive(Debug, PartialEq, Clone)]
	pub struct DisconnectV765 {
		pub reason: Nbt,
	}
}
