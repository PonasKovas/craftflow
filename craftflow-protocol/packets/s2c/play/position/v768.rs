// [
//     "container",
//     [
//         {
//             "name": "teleportId",
//             "type": "varint"
//         },
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
//             "name": "dx",
//             "type": "f64"
//         },
//         {
//             "name": "dy",
//             "type": "f64"
//         },
//         {
//             "name": "dz",
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
//             "type": "PositionUpdateRelatives"
//         }
//     ]
// ]

mcp! {
	#[derive(Debug, PartialEq, Clone)]
	pub struct PositionV768 {
		pub teleport_id: (VarInt),
		pub x: (f64),
		pub y: (f64),
		pub z: (f64),
		pub dx: (f64),
		pub dy: (f64),
		pub dz: (f64),
		pub yaw: (f32),
		pub pitch: (f32),
		pub flags: (PositionUpdateRelatives),
	}
}
