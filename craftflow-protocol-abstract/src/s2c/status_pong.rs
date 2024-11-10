use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, State, WriteResult};
use anyhow::Result;
use craftflow_protocol_versions::{
	s2c::{
		status::{ping::v00005::PingV00005, Ping},
		Status,
	},
	IntoStateEnum, S2C,
};
use shallowclone::ShallowClone;
use std::iter::{once, Once};

/// Response to the [`AbStatusPing`][crate::c2s::AbStatusPing] packet.
#[derive(ShallowClone, Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbStatusPong {
	/// Must be the same number as sent by the client in the [`AbStatusPing`][crate::c2s::AbStatusPing] packet.
	pub id: u64,
}

impl<'a> AbPacketWrite<'a> for AbStatusPong {
	type Direction = S2C<'a>;
	type Iter = Once<Self::Direction>;

	fn convert(&self, _protocol_version: u32, state: State) -> Result<WriteResult<Self::Iter>> {
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

impl<'a> AbPacketNew<'a> for AbStatusPong {
	type Direction = S2C<'a>;
	type Constructor = NoConstructor<Self, S2C<'a>>;

	fn construct(packet: &Self::Direction) -> Result<ConstructorResult<Self, Self::Constructor>> {
		match packet {
			S2C::Status(Status::Ping(Ping::V00005(packet))) => Ok(ConstructorResult::Done(Self {
				id: packet.time as u64,
			})),
			_ => Ok(ConstructorResult::Ignore),
		}
	}
}
