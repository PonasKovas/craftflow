use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, State, WriteResult};
use anyhow::Result;
use craftflow_protocol_versions::{
	c2s::{
		login::{login_acknowledged::v00764::LoginAcknowledgedV00764, LoginAcknowledged},
		Login,
	},
	IntoStateEnum, C2S,
};
use shallowclone::{MakeOwned, ShallowClone};
use std::iter::{once, Once};

/// Acknowledges the end of the login state
#[derive(ShallowClone, MakeOwned, Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbLoginAcknowledge {}

impl<'a> AbPacketWrite<'a> for AbLoginAcknowledge {
	type Direction = C2S<'a>;
	type Iter = Once<Self::Direction>;

	fn convert(&'a self, protocol_version: u32, state: State) -> Result<WriteResult<Self::Iter>> {
		if state != State::Login {
			return Ok(WriteResult::Unsupported);
		}

		let pkt = match protocol_version {
			764.. => LoginAcknowledgedV00764 {}.into_state_enum(),
			_ => return Ok(WriteResult::Unsupported),
		};

		Ok(WriteResult::Success(once(pkt)))
	}
}

impl<'a> AbPacketNew<'a> for AbLoginAcknowledge {
	type Direction = C2S<'a>;
	type Constructor = NoConstructor<AbLoginAcknowledge>;

	fn construct(
		packet: &'a Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor>> {
		Ok(match packet {
			C2S::Login(Login::LoginAcknowledged(pkt)) => match pkt {
				LoginAcknowledged::V00764(_pkt) => ConstructorResult::Done(Self {}),
			},
			_ => ConstructorResult::Ignore,
		})
	}
}
