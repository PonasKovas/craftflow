use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, WriteResult};
use anyhow::Result;
use craftflow_protocol_core::{common_structures::Text, datatypes::AnonymousNbt};
use craftflow_protocol_versions::{
	s2c::{
		configuration::{add_resource_pack::v00767::AddResourcePackV00765, AddResourcePack},
		Configuration,
	},
	IntoStateEnum, S2C,
};
use std::iter::{once, Once};

#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbConfAddResourcePack {
	pub uuid: u128,
	pub url: String,
	pub hash: String,
	pub forced: bool,
	pub prompt_message: Option<Text>,
}

impl AbPacketWrite for AbConfAddResourcePack {
	type Direction = S2C;
	type Iter = Once<Self::Direction>;

	fn convert(self, protocol_version: u32) -> Result<WriteResult<Self::Iter>> {
		let pkt = match protocol_version {
			765.. => AddResourcePackV00765 {
				uuid: self.uuid,
				url: self.url,
				hash: self.hash,
				forced: self.forced,
				prompt_message: self.prompt_message.map(|m| AnonymousNbt { inner: m }),
			}
			.into_state_enum(),
			_ => return Ok(WriteResult::Unsupported),
		};

		Ok(WriteResult::Success(once(pkt)))
	}
}

impl AbPacketNew for AbConfAddResourcePack {
	type Direction = S2C;
	type Constructor = NoConstructor<Self, S2C>;

	fn construct(
		packet: Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor, Self::Direction>> {
		Ok(match packet {
			S2C::Configuration(Configuration::AddResourcePack(AddResourcePack::V00765(pkt))) => {
				ConstructorResult::Done(Self {
					uuid: pkt.uuid,
					url: pkt.url,
					hash: pkt.hash,
					forced: pkt.forced,
					prompt_message: pkt.prompt_message.map(|m| m.inner),
				})
			}
			_ => ConstructorResult::Ignore(packet),
		})
	}
}
