// [
//     "container",
//     [
//         {
//             "name": "sharedSecret",
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
		pub shared_secret: Buffer,
		pub verify_token: Buffer,
	}
}
