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
	#[shallowclone(target = "RegistryDataV00766<'shallowclone, 'b>")]
	pub struct RegistryDataV00766<'a, 'b> {
		pub id: Cow<'a, str>,
		pub entries: Array<'a, VarInt, Entry<'b>>,
	}
}

define_type! {
	#[derive(ShallowClone, Debug, PartialEq, Clone)]
	pub struct Entry<'a> {
		pub key: Cow<'a, str>,
		pub value: Option<AnonymousNbt>,
	}
}
