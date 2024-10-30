use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, State, WriteResult};
use anyhow::Result;
use craftflow_protocol_core::datatypes::RestBuffer;
use craftflow_protocol_versions::{
	s2c::{
		configuration::{custom_payload::v00764::CustomPayloadV00764, CustomPayload},
		Configuration,
	},
	IntoStateEnum, S2C,
};
use std::iter::{once, Once};

/// Sends a plugin request to the client
#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbConfPlugin {
	/// Channel name of the plugin
	pub channel: String,
	/// Any data that the plugin wants to send
	pub data: Vec<u8>,
}

impl AbPacketWrite for AbConfPlugin {
	type Direction = S2C;
	type Iter = Once<Self::Direction>;

	fn convert(self, protocol_version: u32, state: State) -> Result<WriteResult<Self::Iter>> {
		if state != State::Configuration {
			return Ok(WriteResult::Unsupported);
		}

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
	type Direction = S2C;
	type Constructor = NoConstructor<Self, S2C>;

	fn construct(
		packet: Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor, Self::Direction>> {
		Ok(match packet {
			S2C::Configuration(Configuration::CustomPayload(CustomPayload::V00764(pkt))) => {
				ConstructorResult::Done(Self {
					channel: pkt.channel,
					data: pkt.data.0,
				})
			}
			_ => ConstructorResult::Ignore(packet),
		})
	}
}
