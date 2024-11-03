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
//             "name": "playerUUID",
//             "type": [
//                 "option",
//                 "UUID"
//             ]
//         }
//     ]
// ]

define_type! {
	#[derive(ShallowClone, Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
	pub struct LoginStartV00761<'a> {
		pub username: Cow<'a, str>,
		pub player_uuid: Option<u128>,
	}
}
