use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, State, WriteResult};
use anyhow::Result;
use craftflow_protocol_core::datatypes::RestBuffer;
use craftflow_protocol_versions::{
	c2s::{
		configuration::{custom_payload::v00764::CustomPayloadV00764, CustomPayload},
		Configuration,
	},
	IntoStateEnum, C2S,
};
use shallowclone::{MakeOwned, ShallowClone};
use std::{
	borrow::Cow,
	iter::{once, Once},
};

/// A custom plugin message to the server
#[derive(ShallowClone, MakeOwned, Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbConfPlugin<'a> {
	pub channel: Cow<'a, str>,
	pub data: Cow<'a, [u8]>,
}

impl<'a> AbPacketWrite<'a> for AbConfPlugin<'a> {
	type Direction = C2S<'a>;
	type Iter = Once<Self::Direction>;

	fn convert(&'a self, protocol_version: u32, state: State) -> Result<WriteResult<Self::Iter>> {
		if state != State::Configuration {
			return Ok(WriteResult::Unsupported);
		}

		let pkt = match protocol_version {
			764.. => CustomPayloadV00764 {
				channel: self.channel.shallow_clone(),
				data: RestBuffer::from(self.data.shallow_clone()),
			}
			.into_state_enum(),
			_ => return Ok(WriteResult::Unsupported),
		};

		Ok(WriteResult::Success(once(pkt)))
	}
}

impl<'a> AbPacketNew<'a> for AbConfPlugin<'a> {
	type Direction = C2S<'a>;
	type Constructor = NoConstructor<Self, C2S<'a>>;

	fn construct(
		packet: &'a Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor>> {
		Ok(match packet {
			C2S::Configuration(Configuration::CustomPayload(pkt)) => match pkt {
				CustomPayload::V00764(pkt) => ConstructorResult::Done(Self {
					channel: pkt.channel.shallow_clone(),
					data: pkt.data.data.shallow_clone(),
				}),
			},
			_ => ConstructorResult::Ignore,
		})
	}
}
