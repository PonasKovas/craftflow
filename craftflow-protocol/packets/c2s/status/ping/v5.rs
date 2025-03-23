// [
//     "container",
//     [
//         {
//             "name": "time",
//             "type": "i64"
//         }
//     ]
// ]

mcp! {
	#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
	pub struct PingV5 {
		pub time: (i64),
	}
}
