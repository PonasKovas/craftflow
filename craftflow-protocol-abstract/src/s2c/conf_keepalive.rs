use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, State, WriteResult};
use anyhow::Result;
use craftflow_protocol_versions::{
	s2c::{
		configuration::{keep_alive::v00764::KeepAliveV00764, KeepAlive},
		Configuration,
	},
	IntoStateEnum, S2C,
};
use shallowclone::{MakeOwned, ShallowClone};
use std::iter::{once, Once};

#[derive(ShallowClone, MakeOwned, Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbConfKeepAlive {
	pub id: i64,
}

impl<'a> AbPacketWrite<'a> for AbConfKeepAlive {
	type Direction = S2C<'a>;
	type Iter = Once<Self::Direction>;

	fn convert(&'a self, protocol_version: u32, state: State) -> Result<WriteResult<Self::Iter>> {
		if state != State::Configuration {
			return Ok(WriteResult::Unsupported);
		}

		let pkt = match protocol_version {
			764.. => KeepAliveV00764 {
				keep_alive_id: self.id,
			}
			.into_state_enum(),
			_ => return Ok(WriteResult::Unsupported),
		};

		Ok(WriteResult::Success(once(pkt)))
	}
}

impl<'a> AbPacketNew<'a> for AbConfKeepAlive {
	type Direction = S2C<'a>;
	type Constructor = NoConstructor<AbConfKeepAlive>;

	fn construct(
		packet: &'a Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor>> {
		Ok(match packet {
			S2C::Configuration(Configuration::KeepAlive(KeepAlive::V00764(pkt))) => {
				ConstructorResult::Done(Self {
					id: pkt.keep_alive_id,
				})
			}
			_ => ConstructorResult::Ignore,
		})
	}
}
