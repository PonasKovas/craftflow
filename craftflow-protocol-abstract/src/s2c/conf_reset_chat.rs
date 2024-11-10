use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, State, WriteResult};
use anyhow::Result;
use craftflow_protocol_versions::{
	s2c::{
		configuration::{reset_chat::v00766::ResetChatV00766, ResetChat},
		Configuration,
	},
	IntoStateEnum, S2C,
};
use shallowclone::ShallowClone;
use std::iter::{once, Once};

#[derive(ShallowClone, Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbConfResetChat {}

impl<'a> AbPacketWrite<'a> for AbConfResetChat {
	type Direction = S2C<'a>;
	type Iter = Once<Self::Direction>;

	fn convert(&'a self, protocol_version: u32, state: State) -> Result<WriteResult<Self::Iter>> {
		if state != State::Configuration {
			return Ok(WriteResult::Unsupported);
		}

		let pkt = match protocol_version {
			766.. => ResetChatV00766 {}.into_state_enum(),
			_ => return Ok(WriteResult::Unsupported),
		};

		Ok(WriteResult::Success(once(pkt)))
	}
}

impl<'a> AbPacketNew<'a> for AbConfResetChat {
	type Direction = S2C<'a>;
	type Constructor = NoConstructor<Self, S2C<'a>>;

	fn construct(
		packet: &'a Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor>> {
		Ok(match packet {
			S2C::Configuration(Configuration::ResetChat(ResetChat::V00766(_pkt))) => {
				ConstructorResult::Done(Self {})
			}
			_ => ConstructorResult::Ignore,
		})
	}
}
