// [
//     "container",
//     [
//         {
//             "name": "keepAliveId",
//             "type": "i64"
//         }
//     ]
// ]

mcp! {
	#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
	pub struct KeepAliveV764 {
		pub keep_alive_id: (i64),
	}
}
