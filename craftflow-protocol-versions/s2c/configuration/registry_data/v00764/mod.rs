//////////////////////////////////////////////////////////////////////////////////////
// GENERATED // MINECRAFT-DATA COMMIT HASH f1130aea931b948d2ecaecf34ecfe0116bfd4172 //
//////////////////////////////////////////////////////////////////////////////////////

// [
//     "container",
//     [
//         {
//             "name": "codec",
//             "type": "anonymousNbt"
//         }
//     ]
// ]

define_type! {
	#[derive(ShallowClone, MakeOwned, Debug, PartialEq, Clone)]
	pub struct RegistryDataV00764<'a> {
		pub codec: AnonymousNbt<DynNBT<'a>>,
	}
}
