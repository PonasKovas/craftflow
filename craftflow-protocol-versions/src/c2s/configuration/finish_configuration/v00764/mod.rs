
#[allow(unused_imports)]
use crate::types::v00764::*;
#[allow(unused_imports)]
use craftflow_protocol_core::datatypes::*;
#[allow(unused_imports)]
use craftflow_protocol_core::*;

#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq, Default)]
pub struct FinishConfigurationV00764;

impl MCPWrite for FinishConfigurationV00764 {
	fn write(&self, _output: &mut impl std::io::Write) -> Result<usize> {
		Ok(0)
	}
}

impl MCPRead for FinishConfigurationV00764 {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		Ok((input, Self))
	}
}

impl crate::IntoVersionEnum for FinishConfigurationV00764 {
	type Packet = super::super::FinishConfiguration;

	fn into_version_enum(self) -> Self::Packet {
		super::super::FinishConfiguration::V00764(self)
	}
}
impl crate::IntoPacketEnum for FinishConfigurationV00764 {
	type State = super::super::super::Configuration;

	fn into_packet_enum(self) -> Self::State {
		let packet = crate::IntoVersionEnum::into_version_enum(self);
		super::super::super::Configuration::FinishConfiguration(packet)
	}
}
impl crate::IntoStateEnum for FinishConfigurationV00764 {
	type Direction = super::super::super::super::C2S;

	fn into_state_enum(self) -> Self::Direction {
		let state = crate::IntoPacketEnum::into_packet_enum(self);
		super::super::super::super::C2S::Configuration(state)
	}
}
