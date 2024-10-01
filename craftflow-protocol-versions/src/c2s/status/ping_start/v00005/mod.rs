#[allow(unused_imports)]
use craftflow_protocol_core::*;

#[derive(Debug, PartialEq, Default)]
pub struct PingStartV00005;
impl MCPWrite for PingStartV00005 {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		Ok(0)
	}
}
impl MCPRead for PingStartV00005 {
	fn read(input: &[u8]) -> Result<(&[u8], Self)> {
		Ok((input, Self::default()))
	}
}

impl crate::IntoVersionEnum for PingStartV00005 {
	type Packet = super::super::PingStart;

	fn into_version_enum(self) -> Self::Packet {
		super::super::PingStart::V00005(self)
	}
}
impl crate::IntoPacketEnum for PingStartV00005 {
	type State = super::super::super::Status;

	fn into_packet_enum(self) -> Self::State {
		let packet = crate::IntoVersionEnum::into_version_enum(self);
		super::super::super::Status::PingStart(packet)
	}
}
impl crate::IntoStateEnum for PingStartV00005 {
	type Direction = super::super::super::super::C2S;

	fn into_state_enum(self) -> Self::Direction {
		let state = crate::IntoPacketEnum::into_packet_enum(self);
		super::super::super::super::C2S::Status(state)
	}
}
