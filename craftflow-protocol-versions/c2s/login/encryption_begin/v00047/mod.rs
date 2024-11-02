//////////////////////////////////////////////////////////////////////////////////////
// GENERATED // MINECRAFT-DATA COMMIT HASH f1130aea931b948d2ecaecf34ecfe0116bfd4172 //
//////////////////////////////////////////////////////////////////////////////////////

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

define_type! {
	#[derive(Debug, PartialEq, Clone)]
	pub struct EncryptionBeginV00047<'a> {
		pub shared_secret: Buffer<'a, VarInt>,
		pub verify_token: Buffer<'a, VarInt>,
	}
}
