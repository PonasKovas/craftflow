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
//         },
//         {
//             "name": "strictErrorHandling",
//             "type": "bool"
//         }
//     ]
// ]

mcp! {
	#[derive(Debug, PartialEq, Clone)]
	pub struct SuccessV766 {
		pub uuid: u128,
		pub username: String,
		pub properties: Array<Property>,
		pub strict_error_handling: bool,
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
