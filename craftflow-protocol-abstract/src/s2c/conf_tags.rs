use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, State, WriteResult};
use anyhow::Result;
use craftflow_protocol_core::datatypes::{Array, VarInt};
use craftflow_protocol_versions::{
	s2c::{
		configuration::{
			self,
			tags::v00764::{TagContainer, TagsV00764},
		},
		Configuration,
	},
	types::v00767::{tags::Tag, Tags},
	IntoStateEnum, S2C,
};
use shallowclone::ShallowClone;
use std::{
	borrow::Cow,
	iter::{once, Once},
};

#[derive(ShallowClone, Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbConfTags<'a> {
	// these vecs could be specialised cows but im not gonna concern myself with that right now
	// especially considering they wouldn't bring any performance benefits with the current
	// implementation I think, because the data still needs to be converted to the concrete
	// packet format, so allocations are inevitable
	pub tags: Vec<TagRegistry<'a>>,
}

#[derive(ShallowClone, Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct TagRegistry<'a> {
	pub name: Cow<'a, str>,
	pub entries: Vec<TagEntry<'a>>,
}

#[derive(ShallowClone, Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct TagEntry<'a> {
	pub name: Cow<'a, str>,
	pub entries: Cow<'a, [i32]>,
}

impl<'a> AbPacketWrite<'a> for AbConfTags<'a> {
	type Direction = S2C<'a>;
	type Iter = Once<Self::Direction>;

	fn convert(&'a self, protocol_version: u32, state: State) -> Result<WriteResult<Self::Iter>> {
		if state != State::Configuration {
			return Ok(WriteResult::Unsupported);
		}

		let pkt = match protocol_version {
			764.. => TagsV00764 {
				tags: Array::from(
					self.tags
						.iter()
						.map(|reg| TagContainer {
							tag_type: reg.name.shallow_clone(),
							tags: Tags {
								tags: Array::from(
									reg.entries
										.iter()
										.map(|tag| Tag {
											tag_name: tag.name.shallow_clone(),
											entries: Array::from(
												tag.entries
													.iter()
													.map(|x| VarInt(*x))
													.collect::<Vec<_>>(),
											),
										})
										.collect::<Vec<_>>(),
								),
							},
						})
						.collect::<Vec<_>>(),
				),
			}
			.into_state_enum(),
			_ => return Ok(WriteResult::Unsupported),
		};

		Ok(WriteResult::Success(once(pkt)))
	}
}

impl<'a> AbPacketNew<'a> for AbConfTags<'a> {
	type Direction = S2C<'a>;
	type Constructor = NoConstructor<Self, S2C<'a>>;

	fn construct(
		packet: &'a Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor>> {
		Ok(match packet {
			S2C::Configuration(Configuration::Tags(configuration::Tags::V00764(pkt))) => {
				ConstructorResult::Done(Self {
					tags: pkt
						.tags
						.inner
						.iter()
						.map(|reg| TagRegistry {
							name: reg.tag_type.shallow_clone(),
							entries: reg
								.tags
								.tags
								.iter()
								.map(|tag| TagEntry {
									name: tag.tag_name.shallow_clone(),
									entries: tag.entries.iter().map(|e| e.0).collect(),
								})
								.collect(),
						})
						.collect(),
				})
			}
			_ => ConstructorResult::Ignore,
		})
	}
}
