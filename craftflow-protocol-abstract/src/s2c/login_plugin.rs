use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, State, WriteResult};
use anyhow::Result;
use craftflow_protocol_core::datatypes::{RestBuffer, VarInt};
use craftflow_protocol_versions::{
	s2c::{
		login::{login_plugin_request::v00765::LoginPluginRequestV00393, LoginPluginRequest},
		Login,
	},
	IntoStateEnum, S2C,
};
use std::iter::{once, Once};

/// Sends a plugin request to the client
#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbLoginPluginRequest {
	/// ID of the message. The client will respond with a plugin message with a matching ID.
	pub id: u32,
	/// Channel name of the plugin
	pub channel: String,
	/// Any data that the plugin wants to send
	pub data: Vec<u8>,
}

impl AbPacketWrite for AbLoginPluginRequest {
	type Direction = S2C;
	type Iter = Once<Self::Direction>;

	fn convert(self, protocol_version: u32, state: State) -> Result<WriteResult<Self::Iter>> {
		if state != State::Login {
			return Ok(WriteResult::Unsupported);
		}

		let pkt = match protocol_version {
			393.. => LoginPluginRequestV00393 {
				message_id: VarInt(self.id as i32),
				channel: self.channel,
				data: RestBuffer(self.data),
			}
			.into_state_enum(),
			_ => return Ok(WriteResult::Unsupported),
		};

		Ok(WriteResult::Success(once(pkt)))
	}
}

impl AbPacketNew for AbLoginPluginRequest {
	type Direction = S2C;
	type Constructor = NoConstructor<Self, S2C>;

	fn construct(
		packet: Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor, Self::Direction>> {
		Ok(match packet {
			S2C::Login(Login::LoginPluginRequest(LoginPluginRequest::V00393(pkt))) => {
				ConstructorResult::Done(Self {
					id: pkt.message_id.0 as u32,
					channel: pkt.channel,
					data: pkt.data.0,
				})
			}
			_ => ConstructorResult::Ignore(packet),
		})
	}
}
