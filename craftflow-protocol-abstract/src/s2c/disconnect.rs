use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, State, WriteResult};
use anyhow::Result;
use craftflow_protocol_core::{common_structures::Text, datatypes::Json};
use craftflow_protocol_versions::{
	s2c::{self, Configuration, Login},
	IntoStateEnum, S2C,
};
use shallowclone::ShallowClone;
use std::iter::{once, Once};

/// Disconnects the client and displays the given message.
/// Available in login, configuration and play states
#[derive(ShallowClone, Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbDisconnect<'a> {
	pub message: Text<'a>,
}

impl<'a> AbPacketWrite<'a> for AbDisconnect<'a> {
	type Direction = S2C<'a>;
	type Iter = Once<Self::Direction>;

	fn convert(&'a self, _protocol_version: u32, state: State) -> Result<WriteResult<Self::Iter>> {
		let pkt = match state {
			State::Login => s2c::login::disconnect::v00005::DisconnectV00005 {
				reason: Json {
					inner: self.message.shallow_clone(),
				},
			}
			.into_state_enum(),
			State::Configuration => s2c::configuration::disconnect::v00764::DisconnectV00764 {
				reason: Json {
					inner: self.message.shallow_clone(),
				},
			}
			.into_state_enum(),
			_ => return Ok(WriteResult::Unsupported),
		};

		Ok(WriteResult::Success(once(pkt)))
	}
}

impl<'a> AbPacketNew<'a> for AbDisconnect<'a> {
	type Direction = S2C<'a>;
	type Constructor = NoConstructor<Self, S2C<'a>>;

	fn construct(
		packet: &'a Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor>> {
		match packet {
			S2C::Login(Login::Disconnect(s2c::login::Disconnect::V00005(packet))) => {
				Ok(ConstructorResult::Done(Self {
					message: packet.reason.inner.shallow_clone(),
				}))
			}
			S2C::Configuration(Configuration::Disconnect(
				s2c::configuration::Disconnect::V00764(packet),
			)) => Ok(ConstructorResult::Done(Self {
				message: packet.reason.inner.shallow_clone(),
			})),
			_ => Ok(ConstructorResult::Ignore),
		}
	}
}
