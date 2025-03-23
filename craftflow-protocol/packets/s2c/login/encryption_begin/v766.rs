// [
//     "container",
//     [
//         {
//             "name": "serverId",
//             "type": "string"
//         },
//         {
//             "name": "publicKey",
//             "type": [
//                 "buffer",
//                 {
//                     "countType": "varint"
//                 }
//             ]
//         },
//         {
//             "name": "verifyToken",
//             "type": [
//                 "buffer",
//                 {
//                     "countType": "varint"
//                 }
//             ]
//         },
//         {
//             "name": "shouldAuthenticate",
//             "type": "bool"
//         }
//     ]
// ]

mcp! {
	#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
	pub struct EncryptionBeginV766 {
		pub server_id: (String),
		pub public_key: (Buffer),
		pub verify_token: (Buffer),
		pub should_authenticate: (bool),
	}
}
