use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, WriteResult};
use anyhow::Result;
use craftflow_protocol_versions::{
	s2c::{
		configuration::{
			remove_resource_pack::v00767::RemoveResourcePackV00765, RemoveResourcePack,
		},
		Configuration,
	},
	IntoStateEnum, S2C,
};
use std::iter::{once, Once};

#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub enum AbConfRemoveResourcePack {
	RemoveAll,
	RemoveSpecific { uuid: u128 },
}

impl AbPacketWrite for AbConfRemoveResourcePack {
	type Direction = S2C;
	type Iter = Once<Self::Direction>;

	fn convert(self, protocol_version: u32) -> Result<WriteResult<Self::Iter>> {
		let pkt = match protocol_version {
			765.. => RemoveResourcePackV00765 {
				uuid: match self {
					AbConfRemoveResourcePack::RemoveAll => None,
					AbConfRemoveResourcePack::RemoveSpecific { uuid } => Some(uuid),
				},
			}
			.into_state_enum(),
			_ => return Ok(WriteResult::Unsupported),
		};

		Ok(WriteResult::Success(once(pkt)))
	}
}

impl AbPacketNew for AbConfRemoveResourcePack {
	type Direction = S2C;
	type Constructor = NoConstructor<Self, S2C>;

	fn construct(
		packet: Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor, Self::Direction>> {
		Ok(match packet {
			S2C::Configuration(Configuration::RemoveResourcePack(RemoveResourcePack::V00765(
				pkt,
			))) => ConstructorResult::Done(match pkt.uuid {
				Some(uuid) => Self::RemoveSpecific { uuid },
				None => Self::RemoveAll,
			}),
			_ => ConstructorResult::Ignore(packet),
		})
	}
}
