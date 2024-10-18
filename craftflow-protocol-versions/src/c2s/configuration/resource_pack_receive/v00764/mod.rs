#[allow(unused_imports)]
use crate::types::v00764::*;
#[allow(unused_imports)]
use craftflow_protocol_core::datatypes::*;
#[allow(unused_imports)]
use craftflow_protocol_core::*;

#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
pub struct ResourcePackReceiveV00764 {
	pub result: VarInt,
}

impl MCPWrite for ResourcePackReceiveV00764 {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		self.result.write(output)
	}
}

impl MCPRead for ResourcePackReceiveV00764 {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input, result) = VarInt::read(input)?;
		Ok((input, Self { result }))
	}
}

impl crate::IntoVersionEnum for ResourcePackReceiveV00764 {
	type Packet = super::super::ResourcePackReceive;

	fn into_version_enum(self) -> Self::Packet {
		super::super::ResourcePackReceive::V00764(self)
	}
}
impl crate::IntoPacketEnum for ResourcePackReceiveV00764 {
	type State = super::super::super::Configuration;

	fn into_packet_enum(self) -> Self::State {
		let packet = crate::IntoVersionEnum::into_version_enum(self);
		super::super::super::Configuration::ResourcePackReceive(packet)
	}
}
impl crate::IntoStateEnum for ResourcePackReceiveV00764 {
	type Direction = super::super::super::super::C2S;

	fn into_state_enum(self) -> Self::Direction {
		let state = crate::IntoPacketEnum::into_packet_enum(self);
		super::super::super::super::C2S::Configuration(state)
	}
}
