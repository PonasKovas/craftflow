#[path = "parse_packet_info/direction.rs"]
mod direction;
#[path = "parse_packet_info/generics.rs"]
mod generics;
#[path = "parse_packet_info/packet_name.rs"]
mod packet_name;
#[path = "parse_packet_info/state.rs"]
mod state;
#[path = "parse_packet_info/version.rs"]
mod version;

pub use direction::Direction;
pub use generics::Generics;
pub use packet_name::PacketName;
pub use state::State;
pub use version::Version;

use std::{
	collections::HashMap,
	fs::{self, read_dir},
	path::Path,
};

pub type Directions = HashMap<Direction, (Generics, States)>;
pub type States = HashMap<State, (Generics, Packets)>;
pub type Packets = HashMap<PacketName, (Generics, Versions)>;
pub type Versions = HashMap<Version, PacketInfo>;

pub struct PacketInfo {
	pub packet_id: u32,
	pub packet_type: PacketType,
}

pub enum PacketType {
	ReExport {
		version: Version,
	},
	Defined {
		/// the name of the struct/enum
		type_name: String,
		generics: Generics,
	},
}

pub fn parse_packets() -> Directions {
	let mut packets: Directions = HashMap::new();

	// first parse all packets info from the rust source files
	for direction in [Direction::C2S, Direction::S2C] {
		let direction_path = Path::new(direction.mod_name());
		if !direction_path.exists() {
			continue;
		}

		let mut direction_map = HashMap::new();
		let mut direction_generics = Generics::new();

		for state_fs in read_dir(&direction_path).unwrap().map(|f| f.unwrap()) {
			let state = State(state_fs.file_name().into_string().unwrap());

			let mut state_map = HashMap::new();
			let mut state_generics = Generics::new();

			for packet_fs in read_dir(&state_fs.path()).unwrap().map(|f| f.unwrap()) {
				let packet = PacketName(packet_fs.file_name().into_string().unwrap());

				let mut packet_map = HashMap::new();
				let mut packet_generics = Generics::new();

				for version_fs in read_dir(&packet_fs.path()).unwrap().map(|f| f.unwrap()) {
					let version = version_fs.file_name().into_string().unwrap();
					let version = Version(version[1..].parse::<u32>().unwrap());

					let packet_info = parse(direction, &state, &packet, version);

					// if at least one defined packet version has a lifetime
					// the packet has a lifetime
					if let PacketType::Defined { generics, .. } = &packet_info.packet_type {
						packet_generics = packet_generics.union(generics);
					}

					packet_map.insert(version, packet_info);
				}

				state_generics = state_generics.union(&packet_generics);
				state_map.insert(packet.clone(), (packet_generics, packet_map));
			}

			direction_generics = direction_generics.union(&state_generics);
			direction_map.insert(state.clone(), (state_generics, state_map));
		}

		packets.insert(direction, (direction_generics, direction_map));
	}

	packets
}

fn parse(direction: Direction, state: &State, packet: &PacketName, version: Version) -> PacketInfo {
	let path = Path::new(direction.mod_name())
		.join(&state.0)
		.join(&packet.0)
		.join(version.mod_name());

	let packet_id =
		fs::read_to_string(&path.join("packet_id")).expect(&format!("packet_id read {:?}", path));
	let packet_id: u32 = packet_id
		.trim()
		.parse()
		.expect(&format!("packet_id parse {:?}", path));

	let reexport_path = path.join("packet_reexport");
	let name_path = path.join("name");
	let packet_type = if reexport_path.exists() {
		PacketType::ReExport {
			version: Version(
				fs::read_to_string(&reexport_path)
					.expect(&format!("packet_reexport read {:?}", path))
					.trim()
					.parse()
					.expect(&format!("packet_reexport parse {:?}", path)),
			),
		}
	} else {
		let full = fs::read_to_string(&name_path)
			.expect(&format!("name read {:?}", packet))
			.trim()
			.to_owned();

		let (type_name, generics) = if let Some(i) = full.find('<') {
			let type_name = full[..i].to_owned();
			let generics = Generics(
				full[i + 1..full.len() - 1]
					.split(',')
					.map(|s| s.trim().to_owned())
					.collect(),
			);
			(type_name, generics)
		} else {
			(full, Generics::new())
		};

		PacketType::Defined {
			type_name,
			generics,
		}
	};

	PacketInfo {
		packet_id,
		packet_type,
	}
}
