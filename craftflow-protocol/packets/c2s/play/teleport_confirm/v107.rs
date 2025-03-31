// [
//     "container",
//     [
//         {
//             "name": "teleportId",
//             "type": "varint"
//         }
//     ]
// ]

mcp! {
	#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
	pub struct TeleportConfirmV107 {
		pub teleport_id: (VarInt),
	}
}
