use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, State, WriteResult};
use anyhow::Result;
use craftflow_protocol_versions::{
	c2s::{
		status::{ping::v00765::PingV00005, Ping},
		Status,
	},
	IntoStateEnum, C2S,
};
use std::iter::{once, Once};

/// The ping packet that the client sends to the server (in the STATUS state)
/// Should be responded with a `AbStatusPong` packet.
#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbStatusPing {
	/// Any number, used to identify the response
	/// The same will be sent back in the `AbStatusPong` packet
	pub id: u64,
}

impl AbPacketWrite for AbStatusPing {
	type Direction = C2S;
	type Iter = Once<Self::Direction>;

	fn convert(self, _protocol_version: u32, state: State) -> Result<WriteResult<Self::Iter>> {
		if state != State::Status {
			return Ok(WriteResult::Unsupported);
		}

		// This packet is identical in all protocol versions
		Ok(WriteResult::Success(once(
			PingV00005 {
				time: self.id as i64,
			}
			.into_state_enum(),
		)))
	}
}

impl AbPacketNew for AbStatusPing {
	type Direction = C2S;
	type Constructor = NoConstructor<Self, C2S>;

	fn construct(
		packet: Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor, Self::Direction>> {
		match packet {
			C2S::Status(Status::Ping(Ping::V00005(packet))) => Ok(ConstructorResult::Done(Self {
				id: packet.time as u64,
			})),
			_ => Ok(ConstructorResult::Ignore(packet)),
		}
	}
}
