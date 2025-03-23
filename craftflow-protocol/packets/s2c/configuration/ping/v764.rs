// [
//     "container",
//     [
//         {
//             "name": "id",
//             "type": "i32"
//         }
//     ]
// ]

mcp! {
	#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
	pub struct PingV764 {
		pub id: i32,
	}
}
