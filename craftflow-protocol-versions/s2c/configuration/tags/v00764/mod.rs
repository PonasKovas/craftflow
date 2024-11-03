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
	#[shallowclone(target = "TagsV00764<'shallowclone, 'b>")]
	pub struct TagsV00764<'a, 'b> {
		pub tags: Array<'a, VarInt, TagContainer<'b, 'b>>,
	}
}

define_type! {
	#[derive(ShallowClone, Debug, PartialEq, Clone)]
	#[shallowclone(target = "TagContainer<'shallowclone, 'b>")]
	pub struct TagContainer<'a, 'b> {
		pub tag_type: Cow<'a, str>,
		pub tags: Tags<'a, 'b>,
	}
}
