use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, State, WriteResult};
use anyhow::Result;
use craftflow_protocol_versions::{
	c2s::{
		configuration::{pong::v00767::PongV00764, Pong},
		Configuration,
	},
	IntoStateEnum, C2S,
};
use std::iter::{once, Once};

#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbConfPong {
	pub id: i32,
}

impl AbPacketWrite for AbConfPong {
	type Direction = C2S;
	type Iter = Once<Self::Direction>;

	fn convert(self, protocol_version: u32, state: State) -> Result<WriteResult<Self::Iter>> {
		if state != State::Configuration {
			return Ok(WriteResult::Unsupported);
		}

		let pkt = match protocol_version {
			764.. => PongV00764 { id: self.id }.into_state_enum(),
			_ => return Ok(WriteResult::Unsupported),
		};

		Ok(WriteResult::Success(once(pkt)))
	}
}

impl AbPacketNew for AbConfPong {
	type Direction = C2S;
	type Constructor = NoConstructor<Self, C2S>;

	fn construct(
		packet: Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor, Self::Direction>> {
		Ok(match packet {
			C2S::Configuration(Configuration::Pong(pkt)) => match pkt {
				Pong::V00764(pkt) => ConstructorResult::Done(Self { id: pkt.id }),
			},
			_ => ConstructorResult::Ignore(packet),
		})
	}
}
