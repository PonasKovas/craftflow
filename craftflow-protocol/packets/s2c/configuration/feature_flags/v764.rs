// [
//     "container",
//     [
//         {
//             "name": "features",
//             "type": [
//                 "array",
//                 {
//                     "countType": "varint",
//                     "type": "string"
//                 }
//             ]
//         }
//     ]
// ]

mcp! {
	#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
	pub struct FeatureFlagsV764 {
		pub features: (Array<(String)>),
	}
}
