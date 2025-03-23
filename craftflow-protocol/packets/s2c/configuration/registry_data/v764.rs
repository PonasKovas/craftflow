// [
//     "container",
//     [
//         {
//             "name": "codec",
//             "type": "anonymousNbt"
//         }
//     ]
// ]

mcp! {
	#[derive(Debug, PartialEq, Clone)]
	pub struct RegistryDataV764 {
		pub codec: Nbt,
	}
}
