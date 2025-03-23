// [
//     "container",
//     [
//         {
//             "name": "locale",
//             "type": "string"
//         },
//         {
//             "name": "viewDistance",
//             "type": "i8"
//         },
//         {
//             "name": "chatFlags",
//             "type": "varint"
//         },
//         {
//             "name": "chatColors",
//             "type": "bool"
//         },
//         {
//             "name": "skinParts",
//             "type": "u8"
//         },
//         {
//             "name": "mainHand",
//             "type": "varint"
//         },
//         {
//             "name": "enableTextFiltering",
//             "type": "bool"
//         },
//         {
//             "name": "enableServerListing",
//             "type": "bool"
//         },
//         {
//             "name": "particles",
//             "type": "varint"
//         }
//     ]
// ]

mcp! {
	#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
	pub struct SettingsV768 {
		pub locale: (String),
		pub view_distance: (i8),
		pub chat_flags: (VarInt),
		pub chat_colors: (bool),
		pub skin_parts: (u8),
		pub main_hand: (VarInt),
		pub enable_text_filtering: (bool),
		pub enable_server_listing: (bool),
		pub particles: (VarInt),
	}
}
