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
//         }
//     ]
// ]

mcp! {
	#[derive(Debug, PartialEq, Clone)]
	pub struct EncryptionBeginV47 {
		pub server_id: String,
		pub public_key: Buffer,
		pub verify_token: Buffer,
	}
}
