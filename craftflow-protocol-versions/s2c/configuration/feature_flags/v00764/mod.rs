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
	#[derive(ShallowClone, Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
	#[shallowclone(target = "FeatureFlagsV00764<'shallowclone, 'b>")]
	pub struct FeatureFlagsV00764<'a, 'b> {
		pub features: Array<'a, VarInt, Cow<'b, str>>,
	}
}
