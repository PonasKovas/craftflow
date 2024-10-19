use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, WriteResult};
use anyhow::Result;
use craftflow_protocol_versions::{
	s2c::{
		status::{ping::v00765::PingV00005, Ping},
		Status,
	},
	IntoStateEnum, S2C,
};
use std::iter::{once, Once};

/// Response to the [`AbStatusPing`][crate::c2s::AbStatusPing] packet.
#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbStatusPong {
	/// Must be the same number as sent by the client in the [`AbStatusPing`][crate::c2s::AbStatusPing] packet.
	pub id: u64,
}

impl AbPacketWrite for AbStatusPong {
	type Direction = S2C;
	type Iter = Once<Self::Direction>;

	fn convert(self, _protocol_version: u32) -> Result<WriteResult<Self::Iter>> {
		// This packet is identical in all protocol versions

		Ok(WriteResult::Success(once(
			PingV00005 {
				time: self.id as i64,
			}
			.into_state_enum(),
		)))
	}
}

impl AbPacketNew for AbStatusPong {
	type Direction = S2C;
	type Constructor = NoConstructor<Self, S2C>;

	fn construct(
		packet: Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor, Self::Direction>> {
		match packet {
			S2C::Status(Status::Ping(Ping::V00005(packet))) => Ok(ConstructorResult::Done(Self {
				id: packet.time as u64,
			})),
			_ => Ok(ConstructorResult::Ignore(packet)),
		}
	}
}
