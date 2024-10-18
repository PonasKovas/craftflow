#[allow(unused_imports)]
use crate::types::v00764::*;
#[allow(unused_imports)]
use craftflow_protocol_core::datatypes::*;
#[allow(unused_imports)]
use craftflow_protocol_core::*;

#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
pub struct PongV00764 {
	pub id: i32,
}

impl MCPWrite for PongV00764 {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		self.id.write(output)
	}
}

impl MCPRead for PongV00764 {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input, id) = i32::read(input)?;
		Ok((input, Self { id }))
	}
}

impl crate::IntoVersionEnum for PongV00764 {
	type Packet = super::super::Pong;

	fn into_version_enum(self) -> Self::Packet {
		super::super::Pong::V00764(self)
	}
}
impl crate::IntoPacketEnum for PongV00764 {
	type State = super::super::super::Configuration;

	fn into_packet_enum(self) -> Self::State {
		let packet = crate::IntoVersionEnum::into_version_enum(self);
		super::super::super::Configuration::Pong(packet)
	}
}
impl crate::IntoStateEnum for PongV00764 {
	type Direction = super::super::super::super::C2S;

	fn into_state_enum(self) -> Self::Direction {
		let state = crate::IntoPacketEnum::into_packet_enum(self);
		super::super::super::super::C2S::Configuration(state)
	}
}
