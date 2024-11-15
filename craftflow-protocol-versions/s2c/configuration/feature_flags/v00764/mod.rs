//////////////////////////////////////////////////////////////////////////////////////
// GENERATED // MINECRAFT-DATA COMMIT HASH f1130aea931b948d2ecaecf34ecfe0116bfd4172 //
//////////////////////////////////////////////////////////////////////////////////////

// [
//     "container",
//     [
//         {
//             "name": "features",
//             "type": [
//                 "array",
//                 {
//                     "countType": "varint",
//                     "type": "string"
//                 }
//             ]
//         }
//     ]
// ]

define_type! {
	#[derive(ShallowClone, MakeOwned, Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
	pub struct FeatureFlagsV00764<'a> {
		pub features: Array<'a, VarInt, Cow<'a, str>>,
	}
}
