// [
//     "container",
//     [
//         {
//             "name": "messageId",
//             "type": "varint"
//         },
//         {
//             "name": "data",
//             "type": [
//                 "option",
//                 "restBuffer"
//             ]
//         }
//     ]
// ]

mcp! {
	#[derive(Debug, PartialEq, Clone)]
	pub struct LoginPluginResponseV393 {
		pub message_id: VarInt,
		pub data: Option<RestBuffer>,
	}
}
