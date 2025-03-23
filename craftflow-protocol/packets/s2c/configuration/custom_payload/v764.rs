// [
//     "container",
//     [
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
	pub struct CustomPayloadV764 {
		pub channel: String,
		pub data: RestBuffer,
	}
}
