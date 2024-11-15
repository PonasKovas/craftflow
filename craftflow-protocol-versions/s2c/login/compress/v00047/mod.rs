//////////////////////////////////////////////////////////////////////////////////////
// GENERATED // MINECRAFT-DATA COMMIT HASH f1130aea931b948d2ecaecf34ecfe0116bfd4172 //
//////////////////////////////////////////////////////////////////////////////////////

// [
//     "container",
//     [
//         {
//             "name": "threshold",
//             "type": "varint"
//         }
//     ]
// ]

define_type! {
	#[derive(ShallowClone, MakeOwned, Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
	pub struct CompressV00047 {
		pub threshold: VarInt,
	}
}
