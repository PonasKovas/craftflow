use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, State, WriteResult};
use anyhow::Result;
use craftflow_protocol_versions::{
	c2s::{
		login::{login_acknowledged::v00767::LoginAcknowledgedV00764, LoginAcknowledged},
		Login,
	},
	IntoStateEnum, C2S,
};
use std::iter::{once, Once};

/// Acknowledges the end of the login state
#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbLoginAcknowledge {}

impl AbPacketWrite for AbLoginAcknowledge {
	type Direction = C2S;
	type Iter = Once<Self::Direction>;

	fn convert(self, protocol_version: u32, state: State) -> Result<WriteResult<Self::Iter>> {
		if state != State::Login {
			return Ok(WriteResult::Unsupported);
		}

		let pkt = match protocol_version {
			764.. => LoginAcknowledgedV00764.into_state_enum(),
			_ => return Ok(WriteResult::Unsupported),
		};

		Ok(WriteResult::Success(once(pkt)))
	}
}

impl AbPacketNew for AbLoginAcknowledge {
	type Direction = C2S;
	type Constructor = NoConstructor<Self, C2S>;

	fn construct(
		packet: Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor, Self::Direction>> {
		Ok(match packet {
			C2S::Login(Login::LoginAcknowledged(pkt)) => match pkt {
				LoginAcknowledged::V00764(_pkt) => ConstructorResult::Done(Self {}),
			},
			_ => ConstructorResult::Ignore(packet),
		})
	}
}
