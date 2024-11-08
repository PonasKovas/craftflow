//////////////////////////////////////////////////////////////////////////////////////
// GENERATED // MINECRAFT-DATA COMMIT HASH f1130aea931b948d2ecaecf34ecfe0116bfd4172 //
//////////////////////////////////////////////////////////////////////////////////////

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

define_type! {
	#[derive(ShallowClone, Debug, PartialEq, Clone)]
	pub struct SuccessV00759<'a> {
		pub uuid: u128,
		pub username: Cow<'a, str>,
		pub properties: Array<'a, VarInt, Property<'a>>,
	}
}

define_type! {
	#[derive(ShallowClone, Debug, PartialEq, Clone)]
	pub struct Property<'a> {
		pub name: Cow<'a, str>,
		pub value: Cow<'a, str>,
		pub signature: Option<Cow<'a, str>>,
	}
}
