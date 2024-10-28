use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, WriteResult};
use anyhow::Result;
use craftflow_protocol_core::datatypes::{Array, VarInt};
use craftflow_protocol_versions::{
	s2c::{
		configuration::{
			tags::v00767::{TagsRegistry, TagsV00764},
			Tags,
		},
		Configuration,
	},
	types::v00767::tags::Tag,
	IntoStateEnum, S2C,
};
use std::iter::{once, Once};

#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbConfTags {
	pub tags: Vec<TagRegistry>,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct TagRegistry {
	pub name: String,
	pub entries: Vec<TagEntry>,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct TagEntry {
	pub name: String,
	pub entries: Vec<i32>,
}

impl AbPacketWrite for AbConfTags {
	type Direction = S2C;
	type Iter = Once<Self::Direction>;

	fn convert(self, protocol_version: u32) -> Result<WriteResult<Self::Iter>> {
		let pkt = match protocol_version {
			764.. => TagsV00764 {
				registries: Array::new(
					self.tags
						.into_iter()
						.map(|reg| TagsRegistry {
							name: reg.name,
							tags: craftflow_protocol_versions::types::v00767::tags::Tags(
								Array::new(
									reg.entries
										.into_iter()
										.map(|tag| Tag {
											tag_name: tag.name,
											entries: Array::new(
												tag.entries.into_iter().map(VarInt).collect(),
											),
										})
										.collect(),
								),
							),
						})
						.collect(),
				),
			}
			.into_state_enum(),
			_ => return Ok(WriteResult::Unsupported),
		};

		Ok(WriteResult::Success(once(pkt)))
	}
}

impl AbPacketNew for AbConfTags {
	type Direction = S2C;
	type Constructor = NoConstructor<Self, S2C>;

	fn construct(
		packet: Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor, Self::Direction>> {
		Ok(match packet {
			S2C::Configuration(Configuration::Tags(Tags::V00764(pkt))) => {
				ConstructorResult::Done(Self {
					tags: pkt
						.registries
						.data
						.into_iter()
						.map(|reg| TagRegistry {
							name: reg.name,
							entries: reg
								.tags
								.0
								.data
								.into_iter()
								.map(|tag| TagEntry {
									name: tag.tag_name,
									entries: tag.entries.data.into_iter().map(|e| e.0).collect(),
								})
								.collect(),
						})
						.collect(),
				})
			}
			_ => ConstructorResult::Ignore(packet),
		})
	}
}
