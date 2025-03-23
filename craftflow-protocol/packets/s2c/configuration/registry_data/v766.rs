// [
//     "container",
//     [
//         {
//             "name": "id",
//             "type": "string"
//         },
//         {
//             "name": "entries",
//             "type": [
//                 "array",
//                 {
//                     "countType": "varint",
//                     "type": [
//                         "container",
//                         [
//                             {
//                                 "name": "key",
//                                 "type": "string"
//                             },
//                             {
//                                 "name": "value",
//                                 "type": [
//                                     "option",
//                                     "anonymousNbt"
//                                 ]
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
	pub struct RegistryDataV766 {
		pub id: String,
		pub entries: Array<RegistryEntry>,
	}
}

mcp! {
	#[derive(Debug, PartialEq, Clone)]
	pub struct RegistryEntry {
		pub key: String,
		pub value: Option<Nbt>,
	}
}
