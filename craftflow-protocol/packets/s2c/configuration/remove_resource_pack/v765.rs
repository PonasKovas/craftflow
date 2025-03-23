// [
//     "container",
//     [
//         {
//             "name": "uuid",
//             "type": [
//                 "option",
//                 "UUID"
//             ]
//         }
//     ]
// ]

mcp! {
	#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
	pub struct RemoveResourcePackV765 {
		pub uuid: (Option<(u128)>),
	}
}
