//////////////////////////////////////////////////////////////////////////////////////
// GENERATED // MINECRAFT-DATA COMMIT HASH f1130aea931b948d2ecaecf34ecfe0116bfd4172 //
//////////////////////////////////////////////////////////////////////////////////////

// [
//     "array",
//     {
//         "countType": "varint",
//         "type": [
//             "container",
//             [
//                 {
//                     "name": "tagName",
//                     "type": "string"
//                 },
//                 {
//                     "name": "entries",
//                     "type": [
//                         "array",
//                         {
//                             "countType": "varint",
//                             "type": "varint"
//                         }
//                     ]
//                 }
//             ]
//         ]
//     }
// ]

define_type! {
	#[derive(ShallowClone, MakeOwned, Debug, PartialEq, Clone, Hash, PartialOrd, Eq, Ord)]
	pub struct Tags<'a> {
		pub tags: Array<'a, VarInt, Tag<'a>>,
	}
}

define_type! {
	#[derive(ShallowClone, MakeOwned, Debug, PartialEq, Clone, Hash, PartialOrd, Eq, Ord)]
	pub struct Tag<'a> {
		pub tag_name: Cow<'a, str>,
		pub entries: Array<'a, VarInt, VarInt>,
	}
}
