use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, WriteResult};
use anyhow::Result;
use craftflow_protocol_core::datatypes::{RestBuffer, VarInt};
use craftflow_protocol_versions::{
	c2s::{
		login::{login_plugin_response::v00767::LoginPluginResponseV00393, LoginPluginResponse},
		Login,
	},
	IntoStateEnum, C2S,
};
use std::iter::{once, Once};

/// Response to a login plugin request
#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbLoginPluginResponse {
	pub id: i32,
	/// None if the server does not support the plugin
	pub response: Option<Vec<u8>>,
}

impl AbPacketWrite for AbLoginPluginResponse {
	type Direction = C2S;
	type Iter = Once<Self::Direction>;

	fn convert(self, protocol_version: u32) -> Result<WriteResult<Self::Iter>> {
		let pkt = match protocol_version {
			393.. => LoginPluginResponseV00393 {
				message_id: VarInt(self.id),
				data: self.response.map(RestBuffer),
			}
			.into_state_enum(),
			_ => return Ok(WriteResult::Unsupported),
		};

		Ok(WriteResult::Success(once(pkt)))
	}
}

impl AbPacketNew for AbLoginPluginResponse {
	type Direction = C2S;
	type Constructor = NoConstructor<Self, C2S>;

	fn construct(
		packet: Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor, Self::Direction>> {
		Ok(match packet {
			C2S::Login(Login::LoginPluginResponse(pkt)) => match pkt {
				LoginPluginResponse::V00393(pkt) => ConstructorResult::Done(Self {
					id: pkt.message_id.0,
					response: pkt.data.map(|b| b.0),
				}),
			},
			_ => ConstructorResult::Ignore(packet),
		})
	}
}
