//////////////////////////////////////////////////////////////////////////////////////
// GENERATED // MINECRAFT-DATA COMMIT HASH f1130aea931b948d2ecaecf34ecfe0116bfd4172 //
//////////////////////////////////////////////////////////////////////////////////////

// [
//     "container",
//     [
//         {
//             "name": "uuid",
//             "type": "UUID"
//         },
//         {
//             "name": "url",
//             "type": "string"
//         },
//         {
//             "name": "hash",
//             "type": "string"
//         },
//         {
//             "name": "forced",
//             "type": "bool"
//         },
//         {
//             "name": "promptMessage",
//             "type": [
//                 "option",
//                 "anonymousNbt"
//             ]
//         }
//     ]
// ]

define_type! {
	#[derive(ShallowClone, Debug, PartialEq, Clone)]
	pub struct AddResourcePackV00765<'a> {
		pub uuid: u128,
		pub url: Cow<'a, str>,
		pub hash: Cow<'a, str>,
		pub forced: bool,
		pub prompt_message: Option<AnonymousNbt<Text<'a>>>,
	}
}
