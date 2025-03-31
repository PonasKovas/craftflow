// [
//     "container",
//     [
//         {
//             "name": "keepAliveId",
//             "type": "i32"
//         }
//     ]
// ]

mcp! {
	#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
	pub struct KeepAliveV5 {
		pub keep_alive_id: (i32),
	}
}
