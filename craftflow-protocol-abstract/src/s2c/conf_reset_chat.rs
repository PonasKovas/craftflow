use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, WriteResult};
use anyhow::Result;
use craftflow_protocol_versions::{
	s2c::{
		configuration::{reset_chat::v00767::ResetChatV00766, ResetChat},
		Configuration,
	},
	IntoStateEnum, S2C,
};
use std::iter::{once, Once};

#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbConfResetChat {}

impl AbPacketWrite for AbConfResetChat {
	type Direction = S2C;
	type Iter = Once<Self::Direction>;

	fn convert(self, protocol_version: u32) -> Result<WriteResult<Self::Iter>> {
		let pkt = match protocol_version {
			766.. => ResetChatV00766 {}.into_state_enum(),
			_ => return Ok(WriteResult::Unsupported),
		};

		Ok(WriteResult::Success(once(pkt)))
	}
}

impl AbPacketNew for AbConfResetChat {
	type Direction = S2C;
	type Constructor = NoConstructor<Self, S2C>;

	fn construct(
		packet: Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor, Self::Direction>> {
		Ok(match packet {
			S2C::Configuration(Configuration::ResetChat(ResetChat::V00766(_pkt))) => {
				ConstructorResult::Done(Self {})
			}
			_ => ConstructorResult::Ignore(packet),
		})
	}
}
