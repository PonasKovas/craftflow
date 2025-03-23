// [
//     "container",
//     [
//         {
//             "name": "protocolVersion",
//             "type": "varint"
//         },
//         {
//             "name": "serverHost",
//             "type": "string"
//         },
//         {
//             "name": "serverPort",
//             "type": "u16"
//         },
//         {
//             "name": "nextState",
//             "type": "varint"
//         }
//     ]
// ]

mcp! {
	#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
	pub struct SetProtocolV5 {
		pub protocol_version: (VarInt),
		pub server_host: (String),
		pub server_port: (u16),
		pub next_state: (VarInt),
	}
}
