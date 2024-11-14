// This file is raw SLOP. have fun

use crate::{
	AbPacketConstructor, AbPacketNew, AbPacketWrite, ConstructorResult, State, WriteResult,
};
use anyhow::{bail, Context, Result};
use craftflow_nbt::{dyn_nbt, dynamic::DynNBTList, DynNBT};
use craftflow_protocol_core::datatypes::{array::ArrayInner, AnonymousNbt, Array, VarInt};
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

#[derive(ShallowClone, Debug, Clone, PartialEq)]
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
		let nbt = nbt.into_compound().context("expected NBT to be compound")?;

		macro_rules! populate {
			($id:literal) => {{
				let values = match nbt
					.get($id)
					.context(concat!($id, " not found"))?
					.as_compound()
					.context(concat!($id, " expected to be compound"))?
					.get("value")
					.context(concat!($id, " value not found"))?
					.as_list()
					.context(concat!($id, " value expected to be list"))?
				{
					DynNBTList::Owned(vec) => vec,
					DynNBTList::Borrowed(b) => *b,
				};

				let mut values = values
					.into_iter()
					.map(|v| {
						let v = v.as_compound().context("expected value to be compound")?;
						Ok((
							v.get("id")
								.context(concat!($id, " value id not found"))?
								.as_int_nonstrict()
								.context("expected id to be an integer")?,
							v.get("name")
								.context(concat!($id, " value name not found"))?
								.as_string()
								.context("expected name to be string")?
								.clone(), // honestly this is too confusing for me rn, so i just clone here
							// even though im pretty sure it should be doable only using shallow clones
							// but it doesnt really matter than much right now, maybe i will fix it later
							v.get("element")
								.context(concat!($id, " value element not found"))?
								.clone(), // same as above. honestly this whole file is a bit of a slop
							           // and could really use someone with a brighter mind to make it efficient
							           // and actually make use of ShallClone
						))
					})
					.collect::<Result<Vec<_>, Self::Error>>()
					.context(concat!("parsing values of ", $id))?;
				values.sort_unstable_by(|a, b| a.0.cmp(&b.0));
				values
					.into_iter()
					.map(|(_, name, element)| (name, element))
					.collect()
			}};
		}

		let trim_material: IndexMap<_, _> = populate!("minecraft:trim_material");
		let trim_pattern: IndexMap<_, _> = populate!("minecraft:trim_pattern");
		let biome: IndexMap<_, _> = populate!("minecraft:worldgen/biome");
		let chat_type: IndexMap<_, _> = populate!("minecraft:chat_type");
		let damage_type: IndexMap<_, _> = populate!("minecraft:damage_type");
		let dimension_type: IndexMap<_, _> = populate!("minecraft:dimension_type");

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
	fn from(data: &'a AbConfRegistry<'a>) -> Self {
		let mut nbt = HashMap::new();

		macro_rules! populate {
			($data:expr => $code:literal) => {{
				let mut values = Vec::new();
				for (i, (name, element)) in $data.shallow_clone().into_iter().enumerate() {
					values.push(dyn_nbt!({
						"name": name,
						"id": i as i32,
						"element": element,
					}));
				}
				nbt.insert(
					$code.into(),
					dyn_nbt!({
						"type": $code,
						"value": values,
					}),
				);
			}};
		}

		populate!(data.trim_material => "minecraft:trim_material");
		populate!(data.trim_pattern => "minecraft:trim_pattern");
		populate!(data.biome => "minecraft:worldgen/biome");
		populate!(data.chat_type => "minecraft:chat_type");
		populate!(data.damage_type => "minecraft:damage_type");
		populate!(data.dimension_type => "minecraft:dimension_type");

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

#[derive(ShallowClone, Debug, Clone, PartialEq)]
pub enum RegistryPacketIter<'a> {
	V00764(Option<RegistryDataV00764<'a>>),
	V00766 {
		trim_material: Option<IndexMap<Cow<'a, str>, DynNBT<'a>>>,
		trim_pattern: Option<IndexMap<Cow<'a, str>, DynNBT<'a>>>,
		biome: Option<IndexMap<Cow<'a, str>, DynNBT<'a>>>,
		chat_type: Option<IndexMap<Cow<'a, str>, DynNBT<'a>>>,
		damage_type: Option<IndexMap<Cow<'a, str>, DynNBT<'a>>>,
		dimension_type: Option<IndexMap<Cow<'a, str>, DynNBT<'a>>>,
	},
}
impl<'a> Iterator for RegistryPacketIter<'a> {
	type Item = S2C<'a>;

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
								id: $id.into(),
								entries: Array::from(entries),
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

fn convert_entries<'a>(entries: Array<'a, VarInt, Entry>) -> IndexMap<Cow<'a, str>, DynNBT<'a>> {
	let entries = match entries.inner {
		ArrayInner::Owned(o) => o,
		ArrayInner::Borrowed(b) => b.shallow_clone(),
	};

	let mut map = IndexMap::new();
	for entry in entries {
		map.insert(
			entry.key,
			entry
				.value
				.unwrap_or(AnonymousNbt {
					inner: DynNBT::Compound(HashMap::new().into()),
				})
				.inner,
		);
	}

	map
}

#[derive(ShallowClone, Debug, Clone, PartialEq)]
pub struct RegistryConstructor {
	trim_material: Option<IndexMap<Cow<'static, str>, DynNBT<'static>>>,
	trim_pattern: Option<IndexMap<Cow<'static, str>, DynNBT<'static>>>,
	biome: Option<IndexMap<Cow<'static, str>, DynNBT<'static>>>,
	chat_type: Option<IndexMap<Cow<'static, str>, DynNBT<'static>>>,
	damage_type: Option<IndexMap<Cow<'static, str>, DynNBT<'static>>>,
	dimension_type: Option<IndexMap<Cow<'static, str>, DynNBT<'static>>>,
}

impl AbPacketConstructor for Option<RegistryConstructor> {
	type AbPacket = AbConfRegistry<'static>;
	type Direction = S2C<'static>;

	fn next_packet(
		&mut self,
		packet: &Self::Direction,
	) -> Result<ConstructorResult<Self::AbPacket, ()>> {
		let s = self
			.as_mut()
			.expect("ab registry constructor already finished");

		match packet {
			S2C::Configuration(Configuration::RegistryData(RegistryData::V00766(pkt))) => {
				*match &*pkt.id {
					"minecraft:trim_material" => &mut s.trim_material,
					"minecraft:trim_pattern" => &mut s.trim_pattern,
					"minecraft:worldgen/biome" => &mut s.biome,
					"minecraft:chat_type" => &mut s.chat_type,
					"minecraft:damage_type" => &mut s.damage_type,
					"minecraft:dimension_type" => &mut s.dimension_type,
					_ => {
						bail!("Unknown registry id: {:?}", pkt.id)
					} // have to clone here because the constructor may live longer
					  // than the individual packets
				} = Some(convert_entries(pkt.entries.clone()));

				if s.biome.is_some()
					&& s.chat_type.is_some()
					&& s.damage_type.is_some()
					&& s.dimension_type.is_some()
					&& s.trim_material.is_some()
					&& s.trim_pattern.is_some()
				{
					Ok(ConstructorResult::Done(AbConfRegistry {
						trim_material: s.trim_material.take().unwrap(),
						trim_pattern: s.trim_pattern.take().unwrap(),
						biome: s.biome.take().unwrap(),
						chat_type: s.chat_type.take().unwrap(),
						damage_type: s.damage_type.take().unwrap(),
						dimension_type: s.dimension_type.take().unwrap(),
					}))
				} else {
					Ok(ConstructorResult::Continue(()))
				}
			}
			_ => Ok(ConstructorResult::Ignore),
		}
	}
}

impl<'a> AbPacketWrite<'a> for AbConfRegistry<'a> {
	type Direction = S2C<'a>;
	type Iter = RegistryPacketIter<'a>;

	fn convert(&'a self, protocol_version: u32, state: State) -> Result<WriteResult<Self::Iter>> {
		if state != State::Configuration {
			return Ok(WriteResult::Unsupported);
		}

		let pkt = match protocol_version {
			764..766 => RegistryPacketIter::V00764(Some(RegistryDataV00764 {
				codec: AnonymousNbt { inner: self.into() },
			})),
			766.. => RegistryPacketIter::V00766 {
				trim_material: Some(self.trim_material.shallow_clone()),
				trim_pattern: Some(self.trim_pattern.shallow_clone()),
				biome: Some(self.biome.shallow_clone()),
				chat_type: Some(self.chat_type.shallow_clone()),
				damage_type: Some(self.damage_type.shallow_clone()),
				dimension_type: Some(self.dimension_type.shallow_clone()),
			},
			_ => return Ok(WriteResult::Unsupported),
		};

		Ok(WriteResult::Success(pkt))
	}
}

impl<'a> AbPacketNew<'a> for AbConfRegistry<'a> {
	type Direction = S2C<'a>;
	type Constructor = Option<RegistryConstructor>;

	fn construct(
		packet: &'a Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor>> {
		Ok(match packet {
			S2C::Configuration(Configuration::RegistryData(RegistryData::V00764(pkt))) => {
				ConstructorResult::Done(
					pkt.codec
						.inner
						.shallow_clone()
						.try_into()
						.context("invalid registry data")?,
				)
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

				*match &*pkt.id {
					"minecraft:trim_material" => &mut constructor.trim_material,
					"minecraft:trim_pattern" => &mut constructor.trim_pattern,
					"minecraft:worldgen/biome" => &mut constructor.biome,
					"minecraft:chat_type" => &mut constructor.chat_type,
					"minecraft:damage_type" => &mut constructor.damage_type,
					"minecraft:dimension_type" => &mut constructor.dimension_type,
					_ => {
						bail!("Unknown registry id: {:?}", pkt.id)
					}
				} = Some(convert_entries(pkt.entries.clone()));

				ConstructorResult::Continue(Some(constructor))
			}
			_ => ConstructorResult::Ignore,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn default_registry() {
		// this should panic if the default implementation cannot be parsed
		let _packet = AbConfRegistry::default();
	}
}
