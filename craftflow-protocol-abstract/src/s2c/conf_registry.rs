// This file is raw SLOP. have fun

use crate::{
	AbPacketConstructor, AbPacketNew, AbPacketWrite, ConstructorResult, State, WriteResult,
};
use anyhow::{bail, Context, Result};
use craftflow_nbt::{dyn_nbt, DynNBT};
use craftflow_protocol_core::datatypes::{AnonymousNbt, Array, VarInt};
use craftflow_protocol_versions::{
	s2c::{
		configuration::{
			registry_data::{
				v00764::RegistryDataV00764,
				v00766::{Entry, RegistryDataV00766},
			},
			RegistryData,
		},
		Configuration,
	},
	IntoStateEnum, S2C,
};
use indexmap::IndexMap;
use shallowclone::ShallowClone;
use std::{borrow::Cow, collections::HashMap, sync::OnceLock};

// Minecraft SLOP
// why tf is this even being sent
// half of it is not even being used, other half could just be handled completely on the server

#[derive(Debug, Clone, PartialEq)]
pub struct AbConfRegistry<'a> {
	/// Armor trim material registry
	pub trim_material: IndexMap<Cow<'a, str>, DynNBT<'a>>,
	/// Armor trim pattern registry
	pub trim_pattern: IndexMap<Cow<'a, str>, DynNBT<'a>>,
	/// Biome registry
	pub biome: IndexMap<Cow<'a, str>, DynNBT<'a>>,
	/// Chat type registry
	pub chat_type: IndexMap<Cow<'a, str>, DynNBT<'a>>,
	/// Damage type registry
	pub damage_type: IndexMap<Cow<'a, str>, DynNBT<'a>>,
	/// Dimension type registry
	pub dimension_type: IndexMap<Cow<'a, str>, DynNBT<'a>>,
}

impl<'a> TryFrom<DynNBT<'a>> for AbConfRegistry<'a> {
	type Error = anyhow::Error;

	fn try_from(nbt: DynNBT<'a>) -> Result<Self, Self::Error> {
		let mut nbt = nbt.into_compound().context("expected NBT to be compound")?;

		let trim_material: IndexMap<_, _>;
		let trim_pattern: IndexMap<_, _>;
		let biome: IndexMap<_, _>;
		let chat_type: IndexMap<_, _>;
		let damage_type: IndexMap<_, _>;
		let dimension_type: IndexMap<_, _>;

		macro_rules! populate {
			($id:literal => $what:ident) => {{
				let mut values = nbt
					.remove($id)
					.context(concat!($id, " not found"))?
					.into_compound()
					.context(concat!($id, " expected to be compound"))?
					.remove("value")
					.context(concat!($id, " value not found"))?
					.into_list()
					.context(concat!($id, " value expected to be list"))?
					.into_iter()
					.map(|v| {
						let mut v = v.into_compound().context("expected value to be compound")?;
						Ok((
							v.remove("id")
								.context(concat!($id, " value id not found"))?
								.into_int()
								.context("expected id to be int")?,
							v.remove("name")
								.context(concat!($id, " value name not found"))?
								.into_string()
								.context("expected name to be string")?,
							v.remove("element")
								.context(concat!($id, " value element not found"))?,
						))
					})
					.collect::<Result<Vec<_>, Self::Error>>()
					.context(concat!("parsing values of ", $id))?;
				values.sort_unstable_by(|a, b| a.0.cmp(&b.0));
				$what = values
					.into_iter()
					.map(|(_, name, element)| (name, element))
					.collect();
			}};
		}

		populate!("minecraft:trim_material" => trim_material);
		populate!("minecraft:trim_pattern" => trim_pattern);
		populate!("minecraft:worldgen/biome" => biome);
		populate!("minecraft:chat_type" => chat_type);
		populate!("minecraft:damage_type" => damage_type);
		populate!("minecraft:dimension_type" => dimension_type);

		Ok(Self {
			trim_material,
			trim_pattern,
			biome,
			chat_type,
			damage_type,
			dimension_type,
		})
	}
}
impl<'a> From<&'a AbConfRegistry<'a>> for DynNBT<'a> {
	fn from(data: &AbConfRegistry<'a>) -> Self {
		macro_rules! populate {
			($nbt:ident, $data:expr => $code:literal) => {{
				let mut values = Vec::new();
				for (i, (name, element)) in $data.iter().enumerate() {
					values.push(dyn_nbt!({
						"name": name.shallow_clone(),
						"id": i as i32,
						"element": element.shallow_clone(),
					}));
				}
				$nbt.insert(
					$code.into(),
					dyn_nbt!({
						"type": $code,
						"value": values,
					}),
				);
			}};
		}

		let mut nbt = HashMap::new();

		populate!(nbt, &data.trim_material => "minecraft:trim_material");
		populate!(nbt, &data.trim_pattern => "minecraft:trim_pattern");
		populate!(nbt, &data.biome => "minecraft:worldgen/biome");
		populate!(nbt, &data.chat_type => "minecraft:chat_type");
		populate!(nbt, &data.damage_type => "minecraft:damage_type");
		populate!(nbt, &data.dimension_type => "minecraft:dimension_type");

		Self::Compound(nbt.into())
	}
}

impl<'a> AbConfRegistry<'a> {
	pub fn default() -> Self {
		static DEFAULT: OnceLock<AbConfRegistry<'static>> = OnceLock::new();

		let data = DEFAULT.get_or_init(|| {
			let json_data = include_str!(concat!(
				env!("CARGO_MANIFEST_DIR"),
				"/assets/default_registry.json"
			));

			let raw: DynNBT = serde_json::from_str(json_data).unwrap();

			AbConfRegistry::try_from(raw).unwrap()
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
