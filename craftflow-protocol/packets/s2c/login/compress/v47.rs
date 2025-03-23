// [
//     "container",
//     [
//         {
//             "name": "threshold",
//             "type": "varint"
//         }
//     ]
// ]

mcp! {
	#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
	pub struct CompressV47 {
		pub threshold: (VarInt),
	}
}
