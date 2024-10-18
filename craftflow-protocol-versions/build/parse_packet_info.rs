use std::{fs, path::Path};

pub struct PacketInfo {
	pub packet_id: u32,
	pub reexport: Option<String>,
}

pub fn parse_packet_info(path: impl AsRef<Path>) -> PacketInfo {
	let packet_id_path = path.as_ref().join("packet_id");
	let packet_id = fs::read_to_string(&packet_id_path).expect(&format!("{:?}", packet_id_path));
	let packet_id: u32 = packet_id
		.trim()
		.parse()
		.expect(&format!("{:?}", packet_id_path));

	let reexport_path = path.as_ref().join("packet_reexport");
	let reexport = if reexport_path.exists() {
		Some(fs::read_to_string(&reexport_path).expect(&format!("{:?}", packet_id_path)))
	} else {
		None
	};

	PacketInfo {
		reexport: reexport.map(|to| to.trim().to_owned()),
		packet_id,
	}
}
