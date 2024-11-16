use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, State, WriteResult};
use anyhow::Result;
use craftflow_protocol_core::datatypes::VarInt;
use craftflow_protocol_versions::{
	s2c::{
		login::{compress::v00047::CompressV00047, Compress},
		Login,
	},
	IntoStateEnum, S2C,
};
use shallowclone::{MakeOwned, ShallowClone};
use std::option::IntoIter;

/// Sets the compression threshold for this connection
#[derive(ShallowClone, MakeOwned, Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbLoginCompress {
	/// If a packet size is above this threshold, it will be compressed. Negative values will disable compression
	/// By default compression is disabled.
	pub threshold: i32,
}

impl<'a> AbPacketWrite<'a> for AbLoginCompress {
	type Direction = S2C<'a>;
	type Iter = IntoIter<Self::Direction>;

	fn convert(&'a self, protocol_version: u32, state: State) -> Result<WriteResult<Self::Iter>> {
		if state != State::Login {
			return Ok(WriteResult::Unsupported);
		}

		let pkt = match protocol_version {
			..47 => {
				// compression doesn't exist for this version.
				None
			}
			47.. => Some(
				CompressV00047 {
					threshold: VarInt(self.threshold),
				}
				.into_state_enum(),
			),
		};

		Ok(WriteResult::Success(pkt.into_iter()))
	}
}

impl<'a> AbPacketNew<'a> for AbLoginCompress {
	type Direction = S2C<'a>;
	type Constructor = NoConstructor<AbLoginCompress>;

	fn construct(
		packet: &'a Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor>> {
		match packet {
			S2C::Login(Login::Compress(Compress::V00047(packet))) => {
				Ok(ConstructorResult::Done(Self {
					threshold: packet.threshold.0,
				}))
			}
			_ => Ok(ConstructorResult::Ignore),
		}
	}
}
