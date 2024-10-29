use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, WriteResult};
use anyhow::Result;
use craftflow_protocol_versions::{
	c2s::{
		configuration::{
			finish_configuration::v00767::FinishConfigurationV00764, FinishConfiguration,
		},
		Configuration,
	},
	IntoStateEnum, C2S,
};
use std::iter::{once, Once};

/// Finishes the configuration state and moves to Play
#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbConfFinish {}

impl AbPacketWrite for AbConfFinish {
	type Direction = C2S;
	type Iter = Once<Self::Direction>;

	fn convert(self, protocol_version: u32) -> Result<WriteResult<Self::Iter>> {
		let pkt = match protocol_version {
			764.. => FinishConfigurationV00764 {}.into_state_enum(),
			_ => return Ok(WriteResult::Unsupported),
		};

		Ok(WriteResult::Success(once(pkt)))
	}
}

impl AbPacketNew for AbConfFinish {
	type Direction = C2S;
	type Constructor = NoConstructor<Self, C2S>;

	fn construct(
		packet: Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor, Self::Direction>> {
		Ok(match packet {
			C2S::Configuration(Configuration::FinishConfiguration(pkt)) => match pkt {
				FinishConfiguration::V00764(_pkt) => ConstructorResult::Done(Self {}),
			},
			_ => ConstructorResult::Ignore(packet),
		})
	}
}
