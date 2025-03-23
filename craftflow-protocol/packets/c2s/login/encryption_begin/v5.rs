// [
//     "container",
//     [
//         {
//             "name": "sharedSecret",
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
	#[derive(Debug, PartialEq, Clone)]
	pub struct EncryptionBeginV5 {
		pub shared_secret: (Buffer<(i16)>),
		pub verify_token: (Buffer<(i16)>),
	}
}
