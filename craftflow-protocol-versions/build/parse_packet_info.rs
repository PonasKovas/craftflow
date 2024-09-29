use std::{fs, path::Path};

pub enum PacketInfo {
	Defined { packet_id: u32 },
	ReExported { to: String },
}

pub fn parse_packet_info(path: impl AsRef<Path>) -> PacketInfo {
	let text = fs::read_to_string(path.as_ref()).expect(&format!("{:?}", path.as_ref()));

	let mut parts = text.splitn(2, '=');
	let key = parts.next().expect(&format!("{:?}", path.as_ref()));
	let value = parts.next().expect(&format!("{:?}", path.as_ref()));

	match key.trim() {
		"packet_id" => PacketInfo::Defined {
			packet_id: value.trim().parse().expect(&format!("{:?}", path.as_ref())),
		},
		"reexport" => PacketInfo::ReExported {
			to: value.trim().to_owned(),
		},
		_ => panic!("Unknown packet_info key: {}", key),
	}
}
