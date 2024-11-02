//////////////////////////////////////////////////////////////////////////////////////
// GENERATED // MINECRAFT-DATA COMMIT HASH f1130aea931b948d2ecaecf34ecfe0116bfd4172 //
//////////////////////////////////////////////////////////////////////////////////////

// [
//     "container",
//     [
//         {
//             "name": "reason",
//             "type": "string"
//         }
//     ]
// ]

define_type! {
	#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
	pub struct DisconnectV00005<'a> {
		pub reason: Cow<'a, str>,
	}
}
