use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, State, WriteResult};
use anyhow::Result;
use craftflow_protocol_core::{common_structures::Text, datatypes::AnonymousNbt};
use craftflow_protocol_versions::{
	s2c::{
		configuration::{add_resource_pack::v00765::AddResourcePackV00765, AddResourcePack},
		Configuration,
	},
	IntoStateEnum, S2C,
};
use shallowclone::{MakeOwned, ShallowClone};
use std::{
	borrow::Cow,
	iter::{once, Once},
};

#[derive(ShallowClone, MakeOwned, Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbConfAddResourcePack<'a> {
	pub uuid: u128,
	pub url: Cow<'a, str>,
	pub hash: Cow<'a, str>,
	pub forced: bool,
	pub prompt_message: Option<Text<'a>>,
}

impl<'a> AbPacketWrite<'a> for AbConfAddResourcePack<'a> {
	type Direction = S2C<'a>;
	type Iter = Once<Self::Direction>;

	fn convert(&'a self, protocol_version: u32, state: State) -> Result<WriteResult<Self::Iter>> {
		if state != State::Configuration {
			return Ok(WriteResult::Unsupported);
		}

		let pkt = match protocol_version {
			765.. => AddResourcePackV00765 {
				uuid: self.uuid,
				url: self.url.shallow_clone(),
				hash: self.hash.shallow_clone(),
				forced: self.forced,
				prompt_message: self
					.prompt_message
					.shallow_clone()
					.map(|m| AnonymousNbt { inner: m }),
			}
			.into_state_enum(),
			_ => return Ok(WriteResult::Unsupported),
		};

		Ok(WriteResult::Success(once(pkt)))
	}
}

impl<'a> AbPacketNew<'a> for AbConfAddResourcePack<'a> {
	type Direction = S2C<'a>;
	type Constructor = NoConstructor<AbConfAddResourcePack<'static>>;

	fn construct(
		packet: &'a Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor>> {
		Ok(match packet {
			S2C::Configuration(Configuration::AddResourcePack(AddResourcePack::V00765(pkt))) => {
				ConstructorResult::Done(Self {
					uuid: pkt.uuid,
					url: pkt.url.shallow_clone(),
					hash: pkt.hash.shallow_clone(),
					forced: pkt.forced,
					prompt_message: pkt.prompt_message.shallow_clone().map(|m| m.inner),
				})
			}
			_ => ConstructorResult::Ignore,
		})
	}
}
