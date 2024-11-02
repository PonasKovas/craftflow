//////////////////////////////////////////////////////////////////////////////////////
// GENERATED // MINECRAFT-DATA COMMIT HASH f1130aea931b948d2ecaecf34ecfe0116bfd4172 //
//////////////////////////////////////////////////////////////////////////////////////

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

define_type! {
	#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
	pub struct EncryptionBeginV00005<'a> {
		pub server_id: Cow<'a, str>,
		pub public_key: Buffer<'a, i16>,
		pub verify_token: Buffer<'a, i16>,
	}
}
