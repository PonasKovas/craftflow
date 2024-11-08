//////////////////////////////////////////////////////////////////////////////////////
// GENERATED // MINECRAFT-DATA COMMIT HASH f1130aea931b948d2ecaecf34ecfe0116bfd4172 //
//////////////////////////////////////////////////////////////////////////////////////

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

define_type! {
	#[derive(ShallowClone, Debug, PartialEq, Clone)]
	pub struct RegistryDataV00766<'a> {
		pub id: Cow<'a, str>,
		pub entries: Array<'a, VarInt, Entry<'a>>,
	}
}

define_type! {
	#[derive(ShallowClone, Debug, PartialEq, Clone)]
	pub struct Entry<'a> {
		pub key: Cow<'a, str>,
		pub value: Option<AnonymousNbt>,
	}
}
