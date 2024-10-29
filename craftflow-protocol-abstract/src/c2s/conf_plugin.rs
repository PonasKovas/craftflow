use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, WriteResult};
use anyhow::Result;
use craftflow_protocol_core::datatypes::RestBuffer;
use craftflow_protocol_versions::{
	c2s::{
		configuration::{custom_payload::v00767::CustomPayloadV00764, CustomPayload},
		Configuration,
	},
	IntoStateEnum, C2S,
};
use std::iter::{once, Once};

/// A custom plugin message to the server
#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbConfPlugin {
	pub channel: String,
	pub data: Vec<u8>,
}

impl AbPacketWrite for AbConfPlugin {
	type Direction = C2S;
	type Iter = Once<Self::Direction>;

	fn convert(self, protocol_version: u32) -> Result<WriteResult<Self::Iter>> {
		let pkt = match protocol_version {
			764.. => CustomPayloadV00764 {
				channel: self.channel,
				data: RestBuffer(self.data),
			}
			.into_state_enum(),
			_ => return Ok(WriteResult::Unsupported),
		};

		Ok(WriteResult::Success(once(pkt)))
	}
}

impl AbPacketNew for AbConfPlugin {
	type Direction = C2S;
	type Constructor = NoConstructor<Self, C2S>;

	fn construct(
		packet: Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor, Self::Direction>> {
		Ok(match packet {
			C2S::Configuration(Configuration::CustomPayload(pkt)) => match pkt {
				CustomPayload::V00764(pkt) => ConstructorResult::Done(Self {
					channel: pkt.channel,
					data: pkt.data.0,
				}),
			},
			_ => ConstructorResult::Ignore(packet),
		})
	}
}
