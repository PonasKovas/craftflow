use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, State, WriteResult};
use anyhow::Result;
use craftflow_protocol_core::datatypes::{RestBuffer, VarInt};
use craftflow_protocol_versions::{
	c2s::{
		login::{login_plugin_response::v00393::LoginPluginResponseV00393, LoginPluginResponse},
		Login,
	},
	IntoStateEnum, C2S,
};
use shallowclone::{MakeOwned, ShallowClone};
use std::{
	borrow::Cow,
	iter::{once, Once},
};

/// Response to a login plugin request
#[derive(ShallowClone, MakeOwned, Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbLoginPluginResponse<'a> {
	pub id: i32,
	/// None if the server does not support the plugin
	pub response: Option<Cow<'a, [u8]>>,
}

impl<'a> AbPacketWrite<'a> for AbLoginPluginResponse<'a> {
	type Direction = C2S<'a>;
	type Iter = Once<Self::Direction>;

	fn convert(&'a self, protocol_version: u32, state: State) -> Result<WriteResult<Self::Iter>> {
		if state != State::Login {
			return Ok(WriteResult::Unsupported);
		}

		let pkt = match protocol_version {
			393.. => LoginPluginResponseV00393 {
				message_id: VarInt(self.id),
				data: self.response.shallow_clone().map(|x| RestBuffer::from(x)),
			}
			.into_state_enum(),
			_ => return Ok(WriteResult::Unsupported),
		};

		Ok(WriteResult::Success(once(pkt)))
	}
}

impl<'a> AbPacketNew<'a> for AbLoginPluginResponse<'a> {
	type Direction = C2S<'a>;
	type Constructor = NoConstructor<Self, C2S<'a>>;

	fn construct(
		packet: &'a Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor>> {
		Ok(match packet {
			C2S::Login(Login::LoginPluginResponse(pkt)) => match pkt {
				LoginPluginResponse::V00393(pkt) => ConstructorResult::Done(Self {
					id: pkt.message_id.0,
					response: pkt.data.shallow_clone().map(|b| b.data),
				}),
			},
			_ => ConstructorResult::Ignore,
		})
	}
}
