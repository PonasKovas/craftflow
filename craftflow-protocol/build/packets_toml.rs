use crate::{PACKETS_TOML, shared::package_dir};
use indexmap::IndexMap;
use serde::Deserialize;
use std::fs::read_to_string;

mod direction;
mod packet_name;
mod state;
mod r#type;
mod version;

pub use direction::Direction;
pub use packet_name::PacketName;
pub use state::State;
pub use r#type::Type;
pub use version::Version;

type Map<T> = IndexMap<String, T>;

/// Actual TOML structure since toml doesnt allow integers as map keys
#[derive(Deserialize, Debug, PartialEq, Clone)]
struct PacketsTomlInternal {
	/// All supported protocol versions
	pub versions: Vec<u32>,
	/// versions that are identical to others
	pub version_aliases: Map<u32>,
	/// types
	#[serde(rename = "type")]
	pub types: TypesInternal,
	/// direction -> state -> packet -> group version : 	packet id -> versions
	#[serde(flatten)]
	pub packets: Map<Map<Map<Map<Map<Vec<u32>>>>>>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
struct TypesInternal {
	#[serde(default)]
	pub s2c: Map<Map<Map<Vec<u32>>>>,
	#[serde(default)]
	pub c2s: Map<Map<Map<Vec<u32>>>>,
	#[serde(flatten)]
	pub common: Map<Map<Vec<u32>>>,
}

// Same as internal but uses integers instead of strings for some keys to be more convenient
#[derive(Debug, PartialEq, Clone)]
pub struct PacketsToml {
	/// All supported protocol versions
	pub versions: Vec<u32>,
	/// versions that are identical to others
	pub version_aliases: IndexMap<u32, u32>,
	/// types
	pub types: IndexMap<Type, IndexMap<Version, Vec<u32>>>,
	/// direction -> state -> packet -> group version : 	packet id -> versions
	pub packets: IndexMap<
		Direction,
		IndexMap<State, IndexMap<PacketName, IndexMap<Version, IndexMap<u32, Vec<u32>>>>>,
	>,
}

pub fn load() -> PacketsToml {
	let s = read_to_string(package_dir().join(PACKETS_TOML)).expect("packets.toml file not found!");

	let internal: PacketsTomlInternal = toml::from_str(&s).expect("parsing packets.toml");

	let mut types = IndexMap::new();
	types.extend(internal.types.s2c.into_iter().flat_map(|(state, v)| {
		v.into_iter().map(move |(name, v)| {
			(
				Type::Specific {
					direction: Direction::S2C,
					state: State(state.clone()),
					name,
				},
				v.into_iter()
					.map(|(k, v)| {
						(
							Version(k.parse().expect("group version must be valid u32 integer")),
							v,
						)
					})
					.collect(),
			)
		})
	}));
	types.extend(internal.types.c2s.into_iter().flat_map(|(state, v)| {
		v.into_iter().map(move |(name, v)| {
			(
				Type::Specific {
					direction: Direction::C2S,
					state: State(state.clone()),
					name,
				},
				v.into_iter()
					.map(|(k, v)| {
						(
							Version(k.parse().expect("group version must be valid u32 integer")),
							v,
						)
					})
					.collect(),
			)
		})
	}));
	types.extend(internal.types.common.into_iter().map(|(k, v)| {
		(
			Type::Common(k),
			v.into_iter()
				.map(|(k, v)| {
					(
						Version(k.parse().expect("group version must be valid u32 integer")),
						v,
					)
				})
				.collect(),
		)
	}));

	PacketsToml {
		versions: internal.versions,
		version_aliases: internal
			.version_aliases
			.into_iter()
			.map(|(k, v)| (k.parse().expect("version must be valid u32 integer"), v))
			.collect(),
		types,
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
