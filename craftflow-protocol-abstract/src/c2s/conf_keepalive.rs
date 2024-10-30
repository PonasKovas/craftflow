use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, State, WriteResult};
use anyhow::Result;
use craftflow_protocol_versions::{
	c2s::{
		configuration::{keep_alive::v00767::KeepAliveV00764, KeepAlive},
		Configuration,
	},
	IntoStateEnum, C2S,
};
use std::iter::{once, Once};

#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbConfKeepAlive {
	pub id: i64,
}

impl AbPacketWrite for AbConfKeepAlive {
	type Direction = C2S;
	type Iter = Once<Self::Direction>;

	fn convert(self, protocol_version: u32, state: State) -> Result<WriteResult<Self::Iter>> {
		if state != State::Configuration {
			return Ok(WriteResult::Unsupported);
		}

		let pkt = match protocol_version {
			764.. => KeepAliveV00764 {
				keep_alive_id: self.id,
			}
			.into_state_enum(),
			_ => return Ok(WriteResult::Unsupported),
		};

		Ok(WriteResult::Success(once(pkt)))
	}
}

impl AbPacketNew for AbConfKeepAlive {
	type Direction = C2S;
	type Constructor = NoConstructor<Self, C2S>;

	fn construct(
		packet: Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor, Self::Direction>> {
		Ok(match packet {
			C2S::Configuration(Configuration::KeepAlive(pkt)) => match pkt {
				KeepAlive::V00764(pkt) => ConstructorResult::Done(Self {
					id: pkt.keep_alive_id,
				}),
			},
			_ => ConstructorResult::Ignore(packet),
		})
	}
}
