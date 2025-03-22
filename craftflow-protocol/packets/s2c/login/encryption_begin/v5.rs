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
//                     "countType": "i16"
//                 }
//             ]
//         },
//         {
//             "name": "verifyToken",
//             "type": [
//                 "buffer",
//                 {
//                     "countType": "i16"
//                 }
//             ]
//         }
//     ]
// ]

mcp! {
	#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
	pub struct EncryptionBeginV5 {
		pub server_id: String,
		pub public_key: Buffer<DEFAULT_ARRAY_LEN_LIMIT, i16>,
		pub verify_token: Buffer<DEFAULT_ARRAY_LEN_LIMIT, i16>,
	}
}
