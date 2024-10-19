use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, WriteResult};
use anyhow::Result;
use craftflow_protocol_core::common_structures::Text;
use craftflow_protocol_versions::{
	s2c::{
		login::{disconnect::v00765::DisconnectV00005, Disconnect},
		Login,
	},
	IntoStateEnum, S2C,
};
use std::iter::{once, Once};

/// Disconnects the client and displays the given message.
#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbLoginDisconnect {
	pub message: Text,
}

impl AbPacketWrite for AbLoginDisconnect {
	type Direction = S2C;
	type Iter = Once<Self::Direction>;

	fn convert(self, _protocol_version: u32) -> Result<WriteResult<Self::Iter>> {
		// This packet is identical in all protocol versions

		Ok(WriteResult::Success(once(
			DisconnectV00005 {
				reason: serde_json::to_string(&self.message)?,
			}
			.into_state_enum(),
		)))
	}
}

impl AbPacketNew for AbLoginDisconnect {
	type Direction = S2C;
	type Constructor = NoConstructor<Self, S2C>;

	fn construct(
		packet: Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor, Self::Direction>> {
		match packet {
			S2C::Login(Login::Disconnect(Disconnect::V00005(packet))) => {
				Ok(ConstructorResult::Done(Self {
					message: serde_json::from_str(&packet.reason)?,
				}))
			}
			_ => Ok(ConstructorResult::Ignore(packet)),
		}
	}
}
