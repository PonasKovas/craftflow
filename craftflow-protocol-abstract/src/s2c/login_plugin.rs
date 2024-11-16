use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, State, WriteResult};
use anyhow::Result;
use craftflow_protocol_core::datatypes::{RestBuffer, VarInt};
use craftflow_protocol_versions::{
	s2c::{
		login::{login_plugin_request::v00393::LoginPluginRequestV00393, LoginPluginRequest},
		Login,
	},
	IntoStateEnum, S2C,
};
use shallowclone::{MakeOwned, ShallowClone};
use std::{
	borrow::Cow,
	iter::{once, Once},
};

/// Sends a plugin request to the client
#[derive(ShallowClone, MakeOwned, Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbLoginPluginRequest<'a> {
	/// ID of the message. The client will respond with a plugin message with a matching ID.
	pub id: u32,
	/// Channel name of the plugin
	pub channel: Cow<'a, str>,
	/// Any data that the plugin wants to send
	pub data: Cow<'a, [u8]>,
}

impl<'a> AbPacketWrite<'a> for AbLoginPluginRequest<'a> {
	type Direction = S2C<'a>;
	type Iter = Once<Self::Direction>;

	fn convert(&'a self, protocol_version: u32, state: State) -> Result<WriteResult<Self::Iter>> {
		if state != State::Login {
			return Ok(WriteResult::Unsupported);
		}

		let pkt = match protocol_version {
			393.. => LoginPluginRequestV00393 {
				message_id: VarInt(self.id as i32),
				channel: self.channel.shallow_clone(),
				data: RestBuffer::from(self.data.shallow_clone()),
			}
			.into_state_enum(),
			_ => return Ok(WriteResult::Unsupported),
		};

		Ok(WriteResult::Success(once(pkt)))
	}
}

impl<'a> AbPacketNew<'a> for AbLoginPluginRequest<'a> {
	type Direction = S2C<'a>;
	type Constructor = NoConstructor<AbLoginPluginRequest<'static>>;

	fn construct(
		packet: &'a Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor>> {
		Ok(match packet {
			S2C::Login(Login::LoginPluginRequest(LoginPluginRequest::V00393(pkt))) => {
				ConstructorResult::Done(Self {
					id: pkt.message_id.0 as u32,
					channel: pkt.channel.shallow_clone(),
					data: pkt.data.data.shallow_clone(),
				})
			}
			_ => ConstructorResult::Ignore,
		})
	}
}
