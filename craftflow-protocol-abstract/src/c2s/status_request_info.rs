use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, State, WriteResult};
use anyhow::Result;
use craftflow_protocol_versions::{
	c2s::{
		status::{ping_start::v00005::PingStartV00005, PingStart},
		Status,
	},
	IntoStateEnum, C2S,
};
use shallowclone::{MakeOwned, ShallowClone};
use std::iter::{once, Once};

/// Requests server information (MOTD, player count, favicon, etc.) in the STATUS state
#[derive(ShallowClone, MakeOwned, Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbStatusRequestInfo;

impl<'a> AbPacketWrite<'a> for AbStatusRequestInfo {
	type Direction = C2S<'a>;
	type Iter = Once<Self::Direction>;

	fn convert(&'a self, _protocol_version: u32, state: State) -> Result<WriteResult<Self::Iter>> {
		if state != State::Status {
			return Ok(WriteResult::Unsupported);
		}

		// This packet is identical in all protocol versions
		Ok(WriteResult::Success(once(
			PingStartV00005 {}.into_state_enum(),
		)))
	}
}

impl<'a> AbPacketNew<'a> for AbStatusRequestInfo {
	type Direction = C2S<'a>;
	type Constructor = NoConstructor<AbStatusRequestInfo>;

	fn construct(
		packet: &'a Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor>> {
		Ok(match packet {
			C2S::Status(Status::PingStart(PingStart::V00005(_))) => ConstructorResult::Done(Self),
			_ => ConstructorResult::Ignore,
		})
	}
}
