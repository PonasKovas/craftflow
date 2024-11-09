use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, State, WriteResult};
use anyhow::Result;
use craftflow_protocol_versions::{
	s2c::{
		configuration::{
			finish_configuration::v00764::FinishConfigurationV00764, FinishConfiguration,
		},
		Configuration,
	},
	IntoStateEnum, S2C,
};
use std::iter::{once, Once};

#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbConfFinish {}

impl<'a> AbPacketWrite<'a> for AbConfFinish {
	type Direction = S2C<'a>;
	type Iter = Once<Self::Direction>;

	fn convert(self, protocol_version: u32, state: State) -> Result<WriteResult<Self::Iter>> {
		if state != State::Configuration {
			return Ok(WriteResult::Unsupported);
		}

		let pkt = match protocol_version {
			764.. => FinishConfigurationV00764 {}.into_state_enum(),
			_ => return Ok(WriteResult::Unsupported),
		};

		Ok(WriteResult::Success(once(pkt)))
	}
}

impl<'a> AbPacketNew<'a> for AbConfFinish {
	type Direction = S2C<'a>;
	type Constructor = NoConstructor<Self, S2C<'a>>;

	fn construct(
		packet: &'a Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor>> {
		Ok(match packet {
			S2C::Configuration(Configuration::FinishConfiguration(
				FinishConfiguration::V00764(_),
			)) => ConstructorResult::Done(Self {}),
			_ => ConstructorResult::Ignore,
		})
	}
}
