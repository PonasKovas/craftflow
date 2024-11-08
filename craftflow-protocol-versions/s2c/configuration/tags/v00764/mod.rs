//////////////////////////////////////////////////////////////////////////////////////
// GENERATED // MINECRAFT-DATA COMMIT HASH f1130aea931b948d2ecaecf34ecfe0116bfd4172 //
//////////////////////////////////////////////////////////////////////////////////////

// [
//     "container",
//     [
//         {
//             "name": "tags",
//             "type": [
//                 "array",
//                 {
//                     "countType": "varint",
//                     "type": [
//                         "container",
//                         [
//                             {
//                                 "name": "tagType",
//                                 "type": "string"
//                             },
//                             {
//                                 "name": "tags",
//                                 "type": "tags"
//                             }
//                         ]
//                     ]
//                 }
//             ]
//         }
//     ]
// ]

define_type! {
	#[derive(ShallowClone, Debug, PartialEq, Clone)]
	pub struct TagsV00764<'a> {
		pub tags: Array<'a, VarInt, TagContainer<'a>>,
	}
}

define_type! {
	#[derive(ShallowClone, Debug, PartialEq, Clone)]
	pub struct TagContainer<'a> {
		pub tag_type: Cow<'a, str>,
		pub tags: Tags<'a>,
	}
}
