//////////////////////////////////////////////////////////////////////////////////////
// GENERATED // MINECRAFT-DATA COMMIT HASH f1130aea931b948d2ecaecf34ecfe0116bfd4172 //
//////////////////////////////////////////////////////////////////////////////////////

// [
//     "container",
//     [
//         {
//             "name": "reason",
//             "type": "anonymousNbt"
//         }
//     ]
// ]

define_type! {
	#[derive(ShallowClone, MakeOwned, Debug, PartialEq, Clone)]
	pub struct DisconnectV00765<'a> {
		pub reason: AnonymousNbt<Text<'a>>,
	}
}
