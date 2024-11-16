use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, State, WriteResult};
use anyhow::Result;
use craftflow_protocol_versions::{
	s2c::{
		configuration::{
			remove_resource_pack::v00765::RemoveResourcePackV00765, RemoveResourcePack,
		},
		Configuration,
	},
	IntoStateEnum, S2C,
};
use shallowclone::{MakeOwned, ShallowClone};
use std::iter::{once, Once};

#[derive(ShallowClone, MakeOwned, Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub enum AbConfRemoveResourcePack {
	RemoveAll,
	RemoveSpecific { uuid: u128 },
}

impl<'a> AbPacketWrite<'a> for AbConfRemoveResourcePack {
	type Direction = S2C<'a>;
	type Iter = Once<Self::Direction>;

	fn convert(&'a self, protocol_version: u32, state: State) -> Result<WriteResult<Self::Iter>> {
		if state != State::Configuration {
			return Ok(WriteResult::Unsupported);
		}

		let pkt = match protocol_version {
			765.. => RemoveResourcePackV00765 {
				uuid: match self {
					AbConfRemoveResourcePack::RemoveAll => None,
					AbConfRemoveResourcePack::RemoveSpecific { uuid } => Some(*uuid),
				},
			}
			.into_state_enum(),
			_ => return Ok(WriteResult::Unsupported),
		};

		Ok(WriteResult::Success(once(pkt)))
	}
}

impl<'a> AbPacketNew<'a> for AbConfRemoveResourcePack {
	type Direction = S2C<'a>;
	type Constructor = NoConstructor<AbConfRemoveResourcePack>;

	fn construct(
		packet: &'a Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor>> {
		Ok(match packet {
			S2C::Configuration(Configuration::RemoveResourcePack(RemoveResourcePack::V00765(
				pkt,
			))) => ConstructorResult::Done(match pkt.uuid {
				Some(uuid) => Self::RemoveSpecific { uuid },
				None => Self::RemoveAll,
			}),
			_ => ConstructorResult::Ignore,
		})
	}
}
