// [
//     "container",
//     [
//         {
//             "name": "packs",
//             "type": [
//                 "array",
//                 {
//                     "countType": "varint",
//                     "type": [
//                         "container",
//                         [
//                             {
//                                 "name": "namespace",
//                                 "type": "string"
//                             },
//                             {
//                                 "name": "id",
//                                 "type": "string"
//                             },
//                             {
//                                 "name": "version",
//                                 "type": "string"
//                             }
//                         ]
//                     ]
//                 }
//             ]
//         }
//     ]
// ]

mcp! {
	#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
	pub struct SelectKnownPacksV766 {
		pub packs: (Array<(PackInfo)>),
	}
}

mcp! {
	#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
	pub struct PackInfo {
		pub namespace: (String),
		pub id: (String),
		pub version: (String),
	}
}
