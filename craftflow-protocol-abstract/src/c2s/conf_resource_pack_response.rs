use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, State, WriteResult};
use anyhow::{bail, Result};
use craftflow_protocol_core::datatypes::VarInt;
use craftflow_protocol_versions::{
	c2s::{
		configuration::{
			resource_pack_receive::{
				v00764::ResourcePackReceiveV00764, v00767::ResourcePackReceiveV00765,
			},
			ResourcePackReceive,
		},
		Configuration,
	},
	IntoStateEnum, C2S,
};
use std::iter::{once, Once};

#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbConfResourcePackResponse {
	pub uuid: u128,
	pub result: ResourcePackResult,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub enum ResourcePackResult {
	Success = 0,
	Declined,
	FailedToDownload,
	Accepted,
	Downloaded,
	InvalidURL,
	FailedToReload,
	Discarded,
}
impl ResourcePackResult {
	pub fn from_i32(v: i32) -> Result<Self> {
		Ok(match v {
			0 => Self::Success,
			1 => Self::Declined,
			2 => Self::FailedToDownload,
			3 => Self::Accepted,
			4 => Self::Downloaded,
			5 => Self::InvalidURL,
			6 => Self::FailedToReload,
			7 => Self::Discarded,
			other => bail!("invalid resource pack result {other:?}"),
		})
	}
}

impl AbPacketWrite for AbConfResourcePackResponse {
	type Direction = C2S;
	type Iter = Once<Self::Direction>;

	fn convert(self, protocol_version: u32, state: State) -> Result<WriteResult<Self::Iter>> {
		if state != State::Configuration {
			return Ok(WriteResult::Unsupported);
		}

		let pkt = match protocol_version {
			764..765 => ResourcePackReceiveV00764 {
				result: VarInt(self.result as i32),
			}
			.into_state_enum(),
			765.. => ResourcePackReceiveV00765 {
				result: VarInt(self.result as i32),
				uuid: self.uuid,
			}
			.into_state_enum(),
			_ => return Ok(WriteResult::Unsupported),
		};

		Ok(WriteResult::Success(once(pkt)))
	}
}

impl AbPacketNew for AbConfResourcePackResponse {
	type Direction = C2S;
	type Constructor = NoConstructor<Self, C2S>;

	fn construct(
		packet: Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor, Self::Direction>> {
		Ok(match packet {
			C2S::Configuration(Configuration::ResourcePackReceive(pkt)) => match pkt {
				ResourcePackReceive::V00764(pkt) => ConstructorResult::Done(Self {
					uuid: 0,
					result: ResourcePackResult::from_i32(pkt.result.0)?,
				}),
				ResourcePackReceive::V00765(pkt) => ConstructorResult::Done(Self {
					uuid: pkt.uuid,
					result: ResourcePackResult::from_i32(pkt.result.0)?,
				}),
			},
			_ => ConstructorResult::Ignore(packet),
		})
	}
}
