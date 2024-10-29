#[allow(unused_imports)]
use crate::types::v00766::*;
#[allow(unused_imports)]
use craftflow_protocol_core::common_structures::*;
#[allow(unused_imports)]
use craftflow_protocol_core::datatypes::*;
#[allow(unused_imports)]
use craftflow_protocol_core::*;

#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
pub struct ResetChatV00766;

impl MCPWrite for ResetChatV00766 {
	fn write(&self, _output: &mut impl std::io::Write) -> Result<usize> {
		Ok(0)
	}
}

impl MCPRead for ResetChatV00766 {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		Ok((input, Self))
	}
}

impl crate::IntoVersionEnum for ResetChatV00766 {
	type Packet = super::super::ResetChat;

	fn into_version_enum(self) -> Self::Packet {
		super::super::ResetChat::V00766(self)
	}
}
impl crate::IntoPacketEnum for ResetChatV00766 {
	type State = super::super::super::Configuration;

	fn into_packet_enum(self) -> Self::State {
		let packet = crate::IntoVersionEnum::into_version_enum(self);
		super::super::super::Configuration::ResetChat(packet)
	}
}
impl crate::IntoStateEnum for ResetChatV00766 {
	type Direction = super::super::super::super::S2C;

	fn into_state_enum(self) -> Self::Direction {
		let state = crate::IntoPacketEnum::into_packet_enum(self);
		super::super::super::super::S2C::Configuration(state)
	}
}
