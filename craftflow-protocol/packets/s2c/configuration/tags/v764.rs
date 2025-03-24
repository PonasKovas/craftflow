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

mcp! {
	#[derive(Debug, PartialEq, Clone)]
	pub struct TagsV764 {
		pub tags: (Array<(TagContainer)>),
	}
}

mcp! {
	#[derive(Debug, PartialEq, Clone)]
	pub struct TagContainer {
		pub tag_type: (String),
		pub tags: (Tags),
	}
}
