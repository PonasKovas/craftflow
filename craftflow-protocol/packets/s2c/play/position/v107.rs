// [
//     "container",
//     [
//         {
//             "name": "x",
//             "type": "f64"
//         },
//         {
//             "name": "y",
//             "type": "f64"
//         },
//         {
//             "name": "z",
//             "type": "f64"
//         },
//         {
//             "name": "yaw",
//             "type": "f32"
//         },
//         {
//             "name": "pitch",
//             "type": "f32"
//         },
//         {
//             "name": "flags",
//             "type": "i8"
//         },
//         {
//             "name": "teleportId",
//             "type": "varint"
//         }
//     ]
// ]

mcp! {
	#[derive(Debug, PartialEq, Clone, PartialOrd)]
	pub struct PositionV107 {
		pub x: (f64),
		pub y: (f64),
		pub z: (f64),
		pub yaw: (f32),
		pub pitch: (f32),
		pub flags: (i8),
		pub teleport_id: (VarInt),
	}
}
