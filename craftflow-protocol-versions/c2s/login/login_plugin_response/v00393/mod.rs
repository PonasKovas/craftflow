//////////////////////////////////////////////////////////////////////////////////////
// GENERATED // MINECRAFT-DATA COMMIT HASH f1130aea931b948d2ecaecf34ecfe0116bfd4172 //
//////////////////////////////////////////////////////////////////////////////////////

// [
//     "container",
//     [
//         {
//             "name": "messageId",
//             "type": "varint"
//         },
//         {
//             "name": "data",
//             "type": [
//                 "option",
//                 "restBuffer"
//             ]
//         }
//     ]
// ]

define_type! {
	#[derive(ShallowClone, Debug, PartialEq, Clone)]
	pub struct LoginPluginResponseV00393<'a> {
		pub message_id: VarInt,
		pub data: Option<RestBuffer<'a>>,
	}
}
