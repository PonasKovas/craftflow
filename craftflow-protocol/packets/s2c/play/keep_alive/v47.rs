// [
//     "container",
//     [
//         {
//             "name": "keepAliveId",
//             "type": "varint"
//         }
//     ]
// ]

mcp! {
	#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
	pub struct KeepAliveV47 {
		pub keep_alive_id: (VarInt),
	}
}
