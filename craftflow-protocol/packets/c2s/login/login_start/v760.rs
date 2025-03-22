// [
//     "container",
//     [
//         {
//             "name": "username",
//             "type": "string"
//         },
//         {
//             "name": "signature",
//             "type": [
//                 "option",
//                 [
//                     "container",
//                     [
//                         {
//                             "name": "timestamp",
//                             "type": "i64"
//                         },
//                         {
//                             "name": "publicKey",
//                             "type": [
//                                 "buffer",
//                                 {
//                                     "countType": "varint"
//                                 }
//                             ]
//                         },
//                         {
//                             "name": "signature",
//                             "type": [
//                                 "buffer",
//                                 {
//                                     "countType": "varint"
//                                 }
//                             ]
//                         }
//                     ]
//                 ]
//             ]
//         },
//         {
//             "name": "playerUUID",
//             "type": [
//                 "option",
//                 "UUID"
//             ]
//         }
//     ]
// ]

mcp! {
	#[derive(Debug, PartialEq, Clone)]
	pub struct LoginStartV760 {
		pub username: String,
		pub signature: Option<Signature>,
		pub player_uuid: Option<u128>,
	}
}

mcp! {
	#[derive(Debug, PartialEq, Clone)]
	pub struct Signature {
		pub timestamp: i64,
		pub public_key: Buffer,
		pub signature: Buffer,
	}
}
