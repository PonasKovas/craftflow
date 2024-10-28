use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, WriteResult};
use anyhow::Result;
use craftflow_protocol_core::{common_structures::Text, datatypes::Json};
use craftflow_protocol_versions::{
	s2c::{
		configuration::{disconnect::v00764::DisconnectV00764, Disconnect},
		Configuration,
	},
	IntoStateEnum, S2C,
};
use std::iter::{once, Once};

#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbConfDisconnect {
	pub reason: Text,
}

impl AbPacketWrite for AbConfDisconnect {
	type Direction = S2C;
	type Iter = Once<Self::Direction>;

	fn convert(self, protocol_version: u32) -> Result<WriteResult<Self::Iter>> {
		let pkt = match protocol_version {
			764.. => DisconnectV00764 {
				reason: Json { inner: self.reason },
			}
			.into_state_enum(),
			_ => return Ok(WriteResult::Unsupported),
		};

		Ok(WriteResult::Success(once(pkt)))
	}
}

impl AbPacketNew for AbConfDisconnect {
	type Direction = S2C;
	type Constructor = NoConstructor<Self, S2C>;

	fn construct(
		packet: Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor, Self::Direction>> {
		Ok(match packet {
			S2C::Configuration(Configuration::Disconnect(Disconnect::V00764(pkt))) => {
				ConstructorResult::Done(Self {
					reason: pkt.reason.inner,
				})
			}
			_ => ConstructorResult::Ignore(packet),
		})
	}
}
