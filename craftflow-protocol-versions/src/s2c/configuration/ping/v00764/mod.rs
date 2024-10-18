#[allow(unused_imports)]
use craftflow_protocol_core::datatypes::*;
#[allow(unused_imports)]
use craftflow_protocol_core::*;

#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
pub struct PingV00764 {
	pub id: i32,
}

impl MCPWrite for PingV00764 {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		self.id.write(output)
	}
}

impl MCPRead for PingV00764 {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input, id) = i32::read(input)?;
		Ok((input, Self { id }))
	}
}

impl crate::IntoVersionEnum for PingV00764 {
	type Packet = super::super::Ping;

	fn into_version_enum(self) -> Self::Packet {
		super::super::Ping::V00764(self)
	}
}
impl crate::IntoPacketEnum for PingV00764 {
	type State = super::super::super::Configuration;

	fn into_packet_enum(self) -> Self::State {
		let packet = crate::IntoVersionEnum::into_version_enum(self);
		super::super::super::Configuration::Ping(packet)
	}
}
impl crate::IntoStateEnum for PingV00764 {
	type Direction = super::super::super::super::S2C;

	fn into_state_enum(self) -> Self::Direction {
		let state = crate::IntoPacketEnum::into_packet_enum(self);
		super::super::super::super::S2C::Configuration(state)
	}
}