//////////////////////////////////////////////////////////////////////////////////////
// GENERATED // MINECRAFT-DATA COMMIT HASH f1130aea931b948d2ecaecf34ecfe0116bfd4172 //
//////////////////////////////////////////////////////////////////////////////////////

// [
//     "container",
//     [
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
	#[derive(Debug, PartialEq, Clone)]
	pub struct CustomPayloadV00764<'a> {
		pub channel: Cow<'a, str>,
		pub data: RestBuffer<'a>,
	}
}
