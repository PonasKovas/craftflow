// [
//     "container",
//     [
//         {
//             "name": "messageId",
//             "type": "varint"
//         },
//         {
//             "name": "channel",
//             "type": "string"
//         },
//         {
//             "name": "data",
//             "type": "restBuffer"
//         }
//     ]
// ]

mcp! {
	#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
	pub struct LoginPluginRequestV393 {
		pub message_id: (VarInt),
		pub channel: (String),
		pub data: (RestBuffer),
	}
}
