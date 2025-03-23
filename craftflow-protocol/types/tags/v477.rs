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

mcp! {
	#[derive(Debug, PartialEq, Clone)]
	pub struct TagsV477 {
		pub inner: (Array<(TagEntry)>),
	}
}

mcp! {
	#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Eq, Ord)]
	pub struct TagEntry {
		pub tag_name: (String),
		pub entries: (Array<(VarInt)>),
	}
}
