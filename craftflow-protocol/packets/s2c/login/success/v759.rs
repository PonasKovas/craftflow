// [
//     "container",
//     [
//         {
//             "name": "uuid",
//             "type": "UUID"
//         },
//         {
//             "name": "username",
//             "type": "string"
//         },
//         {
//             "name": "properties",
//             "type": [
//                 "array",
//                 {
//                     "countType": "varint",
//                     "type": [
//                         "container",
//                         [
//                             {
//                                 "name": "name",
//                                 "type": "string"
//                             },
//                             {
//                                 "name": "value",
//                                 "type": "string"
//                             },
//                             {
//                                 "name": "signature",
//                                 "type": [
//                                     "option",
//                                     "string"
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
	pub struct SuccessV759 {
		pub uuid: u128,
		pub username: String,
		pub properties: Array<Property, DEFAULT_ARRAY_LEN_LIMIT, VarInt>,
	}
}

mcp! {
	#[derive(Debug, PartialEq, Clone)]
	pub struct Property {
		pub name: String,
		pub value: String,
		pub signature: Option<String>,
	}
}
