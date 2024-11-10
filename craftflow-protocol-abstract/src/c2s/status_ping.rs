use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, State, WriteResult};
use anyhow::Result;
use craftflow_protocol_versions::{
	c2s::{
		status::{ping::v00005::PingV00005, Ping},
		Status,
	},
	IntoStateEnum, C2S,
};
use shallowclone::ShallowClone;
use std::iter::{once, Once};

/// The ping packet that the client sends to the server (in the STATUS state)
/// Should be responded with a `AbStatusPong` packet.
#[derive(ShallowClone, Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbStatusPing {
	/// Any number, used to identify the response
	/// The same will be sent back in the `AbStatusPong` packet
	pub id: u64,
}

impl<'a> AbPacketWrite<'a> for AbStatusPing {
	type Direction = C2S<'a>;
	type Iter = Once<Self::Direction>;

	fn convert(&'a self, _protocol_version: u32, state: State) -> Result<WriteResult<Self::Iter>> {
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

impl<'a> AbPacketNew<'a> for AbStatusPing {
	type Direction = C2S<'a>;
	type Constructor = NoConstructor<Self, C2S<'a>>;

	fn construct(
		packet: &'a Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor>> {
		Ok(match packet {
			C2S::Status(Status::Ping(Ping::V00005(packet))) => ConstructorResult::Done(Self {
				id: packet.time as u64,
			}),
			_ => ConstructorResult::Ignore,
		})
	}
}
