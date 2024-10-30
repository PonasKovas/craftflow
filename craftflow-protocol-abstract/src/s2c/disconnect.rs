use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, State, WriteResult};
use anyhow::Result;
use craftflow_protocol_core::{common_structures::Text, datatypes::Json};
use craftflow_protocol_versions::{
	s2c::{self, Configuration, Login},
	IntoStateEnum, S2C,
};
use std::iter::{once, Once};

/// Disconnects the client and displays the given message.
/// Available in login, configuration and play states
#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbDisconnect {
	pub message: Text,
}

impl AbPacketWrite for AbDisconnect {
	type Direction = S2C;
	type Iter = Once<Self::Direction>;

	fn convert(self, _protocol_version: u32, state: State) -> Result<WriteResult<Self::Iter>> {
		let pkt = match state {
			State::Login => s2c::login::disconnect::v00005::DisconnectV00005 {
				reason: Json {
					inner: self.message,
				},
			}
			.into_state_enum(),
			State::Configuration => s2c::configuration::disconnect::v00764::DisconnectV00764 {
				reason: Json {
					inner: self.message,
				},
			}
			.into_state_enum(),
			_ => return Ok(WriteResult::Unsupported),
		};

		Ok(WriteResult::Success(once(pkt)))
	}
}

impl AbPacketNew for AbDisconnect {
	type Direction = S2C;
	type Constructor = NoConstructor<Self, S2C>;

	fn construct(
		packet: Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor, Self::Direction>> {
		match packet {
			S2C::Login(Login::Disconnect(s2c::login::Disconnect::V00005(packet))) => {
				Ok(ConstructorResult::Done(Self {
					message: packet.reason.inner,
				}))
			}
			S2C::Configuration(Configuration::Disconnect(
				s2c::configuration::Disconnect::V00764(packet),
			)) => Ok(ConstructorResult::Done(Self {
				message: packet.reason.inner,
			})),
			_ => Ok(ConstructorResult::Ignore(packet)),
		}
	}
}
