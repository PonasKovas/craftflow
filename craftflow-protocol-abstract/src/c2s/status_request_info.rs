use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, State, WriteResult};
use anyhow::Result;
use craftflow_protocol_versions::{
	c2s::{
		status::{ping_start::v00765::PingStartV00005, PingStart},
		Status,
	},
	IntoStateEnum, C2S,
};
use std::iter::{once, Once};

/// Requests server information (MOTD, player count, favicon, etc.) in the STATUS state
#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbStatusRequestInfo;

impl AbPacketWrite for AbStatusRequestInfo {
	type Direction = C2S;
	type Iter = Once<Self::Direction>;

	fn convert(self, _protocol_version: u32, state: State) -> Result<WriteResult<Self::Iter>> {
		if state != State::Status {
			return Ok(WriteResult::Unsupported);
		}

		// This packet is identical in all protocol versions
		Ok(WriteResult::Success(once(
			PingStartV00005.into_state_enum(),
		)))
	}
}

impl AbPacketNew for AbStatusRequestInfo {
	type Direction = C2S;
	type Constructor = NoConstructor<Self, C2S>;

	fn construct(
		packet: Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor, Self::Direction>> {
		match packet {
			C2S::Status(Status::PingStart(PingStart::V00005(_packet))) => {
				Ok(ConstructorResult::Done(Self))
			}
			_ => Ok(ConstructorResult::Ignore(packet)),
		}
	}
}
