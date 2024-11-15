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
//             "name": "channel",
//             "type": "string"
//         },
//         {
//             "name": "data",
//             "type": "restBuffer"
//         }
//     ]
// ]

define_type! {
	#[derive(ShallowClone, MakeOwned, Debug, PartialEq, Clone)]
	pub struct LoginPluginRequestV00393<'a> {
		pub message_id: VarInt,
		pub channel: Cow<'a, str>,
		pub data: RestBuffer<'a>,
	}
}
