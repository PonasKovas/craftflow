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
	#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
	pub struct TagsV477(
		Array<(
			TagEntry
		)>
	);
}

mcp! {
	#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
	pub struct TagEntry {
		pub tag_name: (String),
		pub entries: (Array<(VarInt)>),
	}
}
