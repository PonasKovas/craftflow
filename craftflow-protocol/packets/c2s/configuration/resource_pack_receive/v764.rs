// [
//     "container",
//     [
//         {
//             "name": "result",
//             "type": "varint"
//         }
//     ]
// ]

mcp! {
	#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
	pub struct ResourcePackReceiveV764 {
		pub result: VarInt,
	}
}
