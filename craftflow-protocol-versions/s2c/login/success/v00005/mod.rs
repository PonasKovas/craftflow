//////////////////////////////////////////////////////////////////////////////////////
// GENERATED // MINECRAFT-DATA COMMIT HASH f1130aea931b948d2ecaecf34ecfe0116bfd4172 //
//////////////////////////////////////////////////////////////////////////////////////

// [
//     "container",
//     [
//         {
//             "name": "uuid",
//             "type": "string"
//         },
//         {
//             "name": "username",
//             "type": "string"
//         }
//     ]
// ]

define_type! {
	#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
	pub struct SuccessV00005<'a> {
		pub uuid: Cow<'a, str>,
		pub username: Cow<'a, str>,
	}
}
