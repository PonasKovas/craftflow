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
use shallowclone::ShallowClone;
use std::{
	borrow::Cow,
	iter::{once, Once},
};

/// Sends a plugin request to the client
#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbConfPlugin<'a> {
	/// Channel name of the plugin
	pub channel: Cow<'a, str>,
	/// Any data that the plugin wants to send
	pub data: Cow<'a, [u8]>,
}

impl<'a> AbPacketWrite<'a> for AbConfPlugin<'a> {
	type Direction = S2C<'a>;
	type Iter = Once<Self::Direction>;

	fn convert(self, protocol_version: u32, state: State) -> Result<WriteResult<Self::Iter>> {
		if state != State::Configuration {
			return Ok(WriteResult::Unsupported);
		}

		let pkt = match protocol_version {
			764.. => CustomPayloadV00764 {
				channel: self.channel,
				data: RestBuffer::from(self.data),
			}
			.into_state_enum(),
			_ => return Ok(WriteResult::Unsupported),
		};

		Ok(WriteResult::Success(once(pkt)))
	}
}

impl<'a> AbPacketNew<'a> for AbConfPlugin<'a> {
	type Direction = S2C<'a>;
	type Constructor = NoConstructor<Self, S2C<'a>>;

	fn construct(
		packet: &'a Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor>> {
		Ok(match packet {
			S2C::Configuration(Configuration::CustomPayload(CustomPayload::V00764(pkt))) => {
				ConstructorResult::Done(Self {
					channel: pkt.channel.shallow_clone(),
					data: pkt.data.data.shallow_clone(),
				})
			}
			_ => ConstructorResult::Ignore,
		})
	}
}
