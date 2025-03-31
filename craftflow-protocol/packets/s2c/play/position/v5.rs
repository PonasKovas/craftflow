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
//             "name": "onGround",
//             "type": "bool"
//         }
//     ]
// ]

mcp! {
	#[derive(Debug, PartialEq, Clone, PartialOrd)]
	pub struct PositionV5 {
		pub x: (f64),
		pub y: (f64),
		pub z: (f64),
		pub yaw: (f32),
		pub pitch: (f32),
		pub on_ground: (bool),
	}
}
