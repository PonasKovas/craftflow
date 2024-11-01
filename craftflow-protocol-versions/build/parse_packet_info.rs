#[path = "parse_packet_info/direction.rs"]
mod direction;
#[path = "parse_packet_info/packet_name.rs"]
mod packet_name;
#[path = "parse_packet_info/state.rs"]
mod state;
#[path = "parse_packet_info/version.rs"]
mod version;

pub use direction::Direction;
pub use packet_name::PacketName;
pub use state::State;
pub use version::Version;

use std::{
	collections::HashMap,
	fs::{self, read_dir},
	path::Path,
};

pub type HasLifetime = bool;
pub type Directions = HashMap<Direction, (HasLifetime, States)>;
pub type States = HashMap<State, (HasLifetime, Packets)>;
pub type Packets = HashMap<PacketName, (HasLifetime, Versions)>;
pub type Versions = HashMap<Version, PacketInfo>;

pub struct PacketInfo {
	pub direction: Direction,
	pub state: State,
	pub packet_name: PacketName,
	pub version: Version,
	pub packet_id: u32,
	pub packet_type: PacketType,
}

pub enum PacketType {
	ReExport {
		version: Version,
	},
	Defined {
		/// the name of the struct/enum, complete with lifetime generics
		type_name: String,
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
		let mut direction_has_lifetime = false;

		for state_fs in read_dir(&direction_path).unwrap().map(|f| f.unwrap()) {
			if !state_fs.file_type().unwrap().is_dir() {
				continue;
			}

			let state = State(state_fs.file_name().into_string().unwrap());

			let mut state_map = HashMap::new();
			let mut state_has_lifetime = false;

			for packet_fs in read_dir(&state_fs.path()).unwrap().map(|f| f.unwrap()) {
				if !packet_fs.file_type().unwrap().is_dir() {
					continue;
				}

				let packet = PacketName(packet_fs.file_name().into_string().unwrap());

				let mut packet_map = HashMap::new();
				let mut packet_has_lifetime = false;

				for version_fs in read_dir(&packet_fs.path()).unwrap().map(|f| f.unwrap()) {
					if !version_fs.file_type().unwrap().is_dir() {
						continue;
					}

					let version = version_fs.file_name().into_string().unwrap();
					let version = Version(version[1..].parse::<u32>().unwrap());

					let packet_info = parse(direction, &state, &packet, version);

					// if at least one defined packet version has a lifetime
					// the packet has a lifetime
					if let PacketType::Defined { type_name } = &packet_info.packet_type {
						packet_has_lifetime |= type_name.contains("<'a>");
					}

					packet_map.insert(version, packet_info);
				}

				state_map.insert(packet.clone(), (packet_has_lifetime, packet_map));
				state_has_lifetime |= packet_has_lifetime;
			}

			direction_map.insert(state.clone(), (state_has_lifetime, state_map));
			direction_has_lifetime |= state_has_lifetime;
		}

		packets.insert(direction, (direction_has_lifetime, direction_map));
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
		PacketType::Defined {
			type_name: fs::read_to_string(&name_path)
				.expect(&format!("name read {:?}", packet))
				.trim()
				.to_owned(),
		}
	};

	PacketInfo {
		direction,
		state: state.clone(),
		packet_name: packet.clone(),
		version,
		packet_id,
		packet_type,
	}
}
