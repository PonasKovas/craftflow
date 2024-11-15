//////////////////////////////////////////////////////////////////////////////////////
// GENERATED // MINECRAFT-DATA COMMIT HASH f1130aea931b948d2ecaecf34ecfe0116bfd4172 //
//////////////////////////////////////////////////////////////////////////////////////

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

define_type! {
	#[derive(ShallowClone, MakeOwned, Debug, PartialEq, Clone)]
	pub struct LoginStartV00760<'a> {
		pub username: Cow<'a, str>,
		pub signature: Option<Signature<'a>>,
		pub player_uuid: Option<u128>,
	}
}

define_type! {
	#[derive(ShallowClone, MakeOwned, Debug, PartialEq, Clone)]
	pub struct Signature<'a> {
		pub timestamp: i64,
		pub public_key: Buffer<'a, VarInt>,
		pub signature: Buffer<'a, VarInt>,
	}
}
