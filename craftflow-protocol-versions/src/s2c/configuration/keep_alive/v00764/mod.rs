#[allow(unused_imports)]
use craftflow_protocol_core::datatypes::*;
#[allow(unused_imports)]
use craftflow_protocol_core::*;

#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Eq, Ord)]
pub struct KeepAliveV00764 {
	pub keep_alive_id: i64,
}

impl MCPWrite for KeepAliveV00764 {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		self.keep_alive_id.write(output)
	}
}

impl MCPRead for KeepAliveV00764 {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input, keep_alive_id) = i64::read(input)?;
		Ok((input, Self { keep_alive_id }))
	}
}

impl crate::IntoVersionEnum for KeepAliveV00764 {
	type Packet = super::super::KeepAlive;

	fn into_version_enum(self) -> Self::Packet {
		super::super::KeepAlive::V00764(self)
	}
}
impl crate::IntoPacketEnum for KeepAliveV00764 {
	type State = super::super::super::Configuration;

	fn into_packet_enum(self) -> Self::State {
		let packet = crate::IntoVersionEnum::into_version_enum(self);
		super::super::super::Configuration::KeepAlive(packet)
	}
}
impl crate::IntoStateEnum for KeepAliveV00764 {
	type Direction = super::super::super::super::S2C;

	fn into_state_enum(self) -> Self::Direction {
		let state = crate::IntoPacketEnum::into_packet_enum(self);
		super::super::super::super::S2C::Configuration(state)
	}
}
