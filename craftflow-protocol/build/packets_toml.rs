use crate::{PACKETS_TOML, shared::package_dir};
use indexmap::IndexMap;
use serde::Deserialize;
use std::fs::read_to_string;

mod direction;
mod packet_name;
mod state;
mod version;

pub use direction::Direction;
pub use packet_name::PacketName;
pub use state::State;
pub use version::Version;

type Map<T> = IndexMap<String, T>;

/// Actual TOML structure since toml doesnt allow integers as map keys
#[derive(Deserialize, Debug, PartialEq, Clone)]
struct PacketsTomlInternal {
	/// All supported protocol versions
	pub versions: Vec<u32>,
	/// direction -> state -> packet -> group version : 	packet id -> versions
	#[serde(flatten)]
	pub packets: Map<Map<Map<Map<Map<Vec<u32>>>>>>,
}

// Same as internal but uses integers instead of strings for some keys to be more convenient
#[derive(Debug, PartialEq, Clone)]
pub struct PacketsToml {
	/// All supported protocol versions
	pub versions: Vec<u32>,
	/// direction -> state -> packet -> group version : 	packet id -> versions
	pub packets: IndexMap<
		Direction,
		IndexMap<State, IndexMap<PacketName, IndexMap<Version, IndexMap<u32, Vec<u32>>>>>,
	>,
}

pub fn load() -> PacketsToml {
	let s = read_to_string(package_dir().join(PACKETS_TOML)).expect("packets.toml file not found!");

	let internal: PacketsTomlInternal = toml::from_str(&s).expect("parsing packets.toml");

	PacketsToml {
		versions: internal.versions,
		packets: internal
			.packets
			.into_iter()
			.map(|(k, v)| {
				(
					Direction::new(&k),
					v.into_iter()
						.map(|(k, v)| {
							(
								State(k),
								v.into_iter()
									.map(|(k, v)| {
										(
											PacketName(k),
											v.into_iter()
												.map(|(k, v)| {
													(
														Version(k.parse().expect(
															"group version must be valid u32 integer",
														)),
														v.into_iter()
															.map(|(k, v)| {
																(
																	k.parse().expect(
																		"packet id must be valid u32 integer",
																	),
																	v,
																)
															})
															.collect(),
													)
												})
												.collect(),
										)
									})
									.collect(),
							)
						})
						.collect(),
				)
			})
			.collect(),
	}
}
