// This file is raw SLOP. have fun

use crate::{
	AbPacketConstructor, AbPacketNew, AbPacketWrite, ConstructorResult, State, WriteResult,
};
use anyhow::{bail, Result};
use craftflow_nbt::DynNBT;
use craftflow_protocol_core::datatypes::{AnonymousNbt, Array, VarInt};
use craftflow_protocol_versions::{
	s2c::{
		configuration::{
			registry_data::{
				v00765::{
					InnerRegistryStructure, RegistryDataV00764, RegistryStructure,
					RegistryValueStructure,
				},
				v00767::{Entry, RegistryDataV00766},
			},
			RegistryData,
		},
		Configuration,
	},
	IntoStateEnum, S2C,
};
use indexmap::IndexMap;
use std::{collections::HashMap, sync::OnceLock};

// Minecraft SLOP
// why tf is this even being sent
// half of it is not even being used, other half could just be handled completely on the server

#[derive(Debug, Clone, PartialEq)]
pub struct AbConfRegistry {
	/// Armor trim material registry
	pub trim_material: IndexMap<String, DynNBT>,
	/// Armor trim pattern registry
	pub trim_pattern: IndexMap<String, DynNBT>,
	/// Biome registry
	pub biome: IndexMap<String, DynNBT>,
	/// Chat type registry
	pub chat_type: IndexMap<String, DynNBT>,
	/// Damage type registry
	pub damage_type: IndexMap<String, DynNBT>,
	/// Dimension type registry
	pub dimension_type: IndexMap<String, DynNBT>,
}

impl From<RegistryStructure> for AbConfRegistry {
	fn from(mut data: RegistryStructure) -> Self {
		let mut trim_material = IndexMap::new();
		let mut trim_pattern = IndexMap::new();
		let mut biome = IndexMap::new();
		let mut chat_type = IndexMap::new();
		let mut damage_type = IndexMap::new();
		let mut dimension_type = IndexMap::new();

		macro_rules! populate {
			($what:ident) => {
				data.$what.value.sort_unstable_by(|a, b| a.id.cmp(&b.id));
				for value in data.$what.value {
					$what.insert(value.name, value.element);
				}
			};
		}

		populate!(trim_material);
		populate!(trim_pattern);
		populate!(biome);
		populate!(chat_type);
		populate!(damage_type);
		populate!(dimension_type);

		Self {
			trim_material,
			trim_pattern,
			biome,
			chat_type,
			damage_type,
			dimension_type,
		}
	}
}
impl From<AbConfRegistry> for RegistryStructure {
	fn from(data: AbConfRegistry) -> Self {
		let mut trim_material = InnerRegistryStructure {
			registry_type: "minecraft:trim_material".to_string(),
			value: Vec::new(),
		};
		let mut trim_pattern = InnerRegistryStructure {
			registry_type: "minecraft:trim_pattern".to_string(),
			value: Vec::new(),
		};
		let mut biome = InnerRegistryStructure {
			registry_type: "minecraft:worldgen/biome".to_string(),
			value: Vec::new(),
		};
		let mut chat_type = InnerRegistryStructure {
			registry_type: "minecraft:chat_type".to_string(),
			value: Vec::new(),
		};
		let mut damage_type = InnerRegistryStructure {
			registry_type: "minecraft:damage_type".to_string(),
			value: Vec::new(),
		};
		let mut dimension_type = InnerRegistryStructure {
			registry_type: "minecraft:dimension_type".to_string(),
			value: Vec::new(),
		};

		macro_rules! populate {
			($what:ident) => {
				for (i, (name, element)) in data.$what.into_iter().enumerate() {
					$what.value.push(RegistryValueStructure {
						name,
						id: i as i32,
						element,
					});
				}
			};
		}

		populate!(trim_material);
		populate!(trim_pattern);
		populate!(biome);
		populate!(chat_type);
		populate!(damage_type);
		populate!(dimension_type);

		Self {
			trim_material,
			trim_pattern,
			biome,
			chat_type,
			damage_type,
			dimension_type,
		}
	}
}

impl AbConfRegistry {
	pub fn default() -> Self {
		static DEFAULT: OnceLock<AbConfRegistry> = OnceLock::new();

		let data = DEFAULT.get_or_init(|| {
			let json_data = include_str!(concat!(
				env!("CARGO_MANIFEST_DIR"),
				"/assets/default_registry.json"
			));

			let raw: RegistryStructure = serde_json::from_str(json_data).unwrap();

			raw.into()
		});

		data.clone()
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum RegistryPacketIter {
	V00764(Option<RegistryDataV00764>),
	V00766 {
		trim_material: Option<IndexMap<String, DynNBT>>,
		trim_pattern: Option<IndexMap<String, DynNBT>>,
		biome: Option<IndexMap<String, DynNBT>>,
		chat_type: Option<IndexMap<String, DynNBT>>,
		damage_type: Option<IndexMap<String, DynNBT>>,
		dimension_type: Option<IndexMap<String, DynNBT>>,
	},
}
impl Iterator for RegistryPacketIter {
	type Item = S2C;

	fn next(&mut self) -> Option<Self::Item> {
		match self {
			RegistryPacketIter::V00764(pkt) => pkt.take().map(|pkt| pkt.into_state_enum()),
			RegistryPacketIter::V00766 {
				trim_material,
				trim_pattern,
				biome,
				chat_type,
				damage_type,
				dimension_type,
			} => {
				macro_rules! convert {
					($what:ident, $id:literal) => {{
						let mut entries = Vec::with_capacity($what.len());
						for (key, value) in $what {
							entries.push(Entry {
								key,
								value: Some(AnonymousNbt { inner: value }),
							});
						}

						Some(
							RegistryDataV00766 {
								id: $id.to_string(),
								entries: Array::new(entries),
							}
							.into_state_enum(),
						)
					}};
				}

				if let Some(trim_material) = trim_material.take() {
					convert!(trim_material, "minecraft:trim_material")
				} else if let Some(trim_pattern) = trim_pattern.take() {
					convert!(trim_pattern, "minecraft:trim_pattern")
				} else if let Some(biome) = biome.take() {
					convert!(biome, "minecraft:worldgen/biome")
				} else if let Some(chat_type) = chat_type.take() {
					convert!(chat_type, "minecraft:chat_type")
				} else if let Some(damage_type) = damage_type.take() {
					convert!(damage_type, "minecraft:damage_type")
				} else if let Some(dimension_type) = dimension_type.take() {
					convert!(dimension_type, "minecraft:dimension_type")
				} else {
					None
				}
			}
		}
	}
}

fn convert_entries(entries: Array<VarInt, Entry>) -> IndexMap<String, DynNBT> {
	entries
		.data
		.into_iter()
		.map(|entry| {
			(
				entry.key,
				entry
					.value
					.unwrap_or(AnonymousNbt {
						inner: DynNBT::Compound(HashMap::new()),
					})
					.inner,
			)
		})
		.collect()
}

#[derive(Debug, Clone, PartialEq)]
pub struct RegistryConstructor {
	trim_material: Option<IndexMap<String, DynNBT>>,
	trim_pattern: Option<IndexMap<String, DynNBT>>,
	biome: Option<IndexMap<String, DynNBT>>,
	chat_type: Option<IndexMap<String, DynNBT>>,
	damage_type: Option<IndexMap<String, DynNBT>>,
	dimension_type: Option<IndexMap<String, DynNBT>>,
}
impl AbPacketConstructor for RegistryConstructor {
	type AbPacket = AbConfRegistry;
	type Direction = S2C;

	fn next_packet(
		mut self: Box<Self>,
		packet: Self::Direction,
	) -> Result<
		ConstructorResult<
			Self::AbPacket,
			Box<
				dyn AbPacketConstructor<AbPacket = Self::AbPacket, Direction = Self::Direction>
					+ Send
					+ Sync,
			>,
			(
				Box<
					dyn AbPacketConstructor<AbPacket = Self::AbPacket, Direction = Self::Direction>
						+ Send
						+ Sync,
				>,
				Self::Direction,
			),
		>,
	> {
		match packet {
			S2C::Configuration(Configuration::RegistryData(RegistryData::V00766(pkt))) => {
				*match pkt.id.as_str() {
					"minecraft:trim_material" => &mut self.trim_material,
					"minecraft:trim_pattern" => &mut self.trim_pattern,
					"minecraft:worldgen/biome" => &mut self.biome,
					"minecraft:chat_type" => &mut self.chat_type,
					"minecraft:damage_type" => &mut self.damage_type,
					"minecraft:dimension_type" => &mut self.dimension_type,
					_ => {
						bail!("Unknown registry id: {:?}", pkt.id)
					}
				} = Some(convert_entries(pkt.entries));

				if self.biome.is_some()
					&& self.chat_type.is_some()
					&& self.damage_type.is_some()
					&& self.dimension_type.is_some()
					&& self.trim_material.is_some()
					&& self.trim_pattern.is_some()
				{
					Ok(ConstructorResult::Done(AbConfRegistry {
						trim_material: self.trim_material.unwrap(),
						trim_pattern: self.trim_pattern.unwrap(),
						biome: self.biome.unwrap(),
						chat_type: self.chat_type.unwrap(),
						damage_type: self.damage_type.unwrap(),
						dimension_type: self.dimension_type.unwrap(),
					}))
				} else {
					Ok(ConstructorResult::Continue(self))
				}
			}
			_ => Ok(ConstructorResult::Ignore((self, packet))),
		}
	}
}

impl AbPacketWrite for AbConfRegistry {
	type Direction = S2C;
	type Iter = RegistryPacketIter;

	fn convert(self, protocol_version: u32, state: State) -> Result<WriteResult<Self::Iter>> {
		if state != State::Configuration {
			return Ok(WriteResult::Unsupported);
		}

		let pkt = match protocol_version {
			764..766 => RegistryPacketIter::V00764(Some(RegistryDataV00764 {
				inner: AnonymousNbt { inner: self.into() },
			})),
			766.. => RegistryPacketIter::V00766 {
				trim_material: Some(self.trim_material),
				trim_pattern: Some(self.trim_pattern),
				biome: Some(self.biome),
				chat_type: Some(self.chat_type),
				damage_type: Some(self.damage_type),
				dimension_type: Some(self.dimension_type),
			},
			_ => return Ok(WriteResult::Unsupported),
		};

		Ok(WriteResult::Success(pkt))
	}
}

impl AbPacketNew for AbConfRegistry {
	type Direction = S2C;
	type Constructor = RegistryConstructor;

	fn construct(
		packet: Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor, Self::Direction>> {
		Ok(match packet {
			S2C::Configuration(Configuration::RegistryData(RegistryData::V00764(pkt))) => {
				ConstructorResult::Done(pkt.inner.inner.into())
			}
			S2C::Configuration(Configuration::RegistryData(RegistryData::V00766(pkt))) => {
				let mut constructor = RegistryConstructor {
					trim_material: None,
					trim_pattern: None,
					biome: None,
					chat_type: None,
					damage_type: None,
					dimension_type: None,
				};

				*match pkt.id.as_str() {
					"minecraft:trim_material" => &mut constructor.trim_material,
					"minecraft:trim_pattern" => &mut constructor.trim_pattern,
					"minecraft:worldgen/biome" => &mut constructor.biome,
					"minecraft:chat_type" => &mut constructor.chat_type,
					"minecraft:damage_type" => &mut constructor.damage_type,
					"minecraft:dimension_type" => &mut constructor.dimension_type,
					_ => {
						bail!("Unknown registry id: {:?}", pkt.id)
					}
				} = Some(convert_entries(pkt.entries));

				ConstructorResult::Continue(constructor)
			}
			_ => ConstructorResult::Ignore(packet),
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn default_registry() {
		let _packet = AbConfRegistry::default();
	}
}
