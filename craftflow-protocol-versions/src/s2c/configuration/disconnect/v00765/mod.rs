use common_structures::Text;
#[allow(unused_imports)]
use craftflow_protocol_core::datatypes::*;
#[allow(unused_imports)]
use craftflow_protocol_core::*;

#[derive(Debug, PartialEq, Clone)]
pub struct DisconnectV00765 {
	pub reason: AnonymousNbt<Text>,
}

impl MCPWrite for DisconnectV00765 {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		self.reason.write(output)
	}
}

impl MCPRead for DisconnectV00765 {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input, reason) = AnonymousNbt::read(input)?;
		Ok((input, Self { reason }))
	}
}

impl crate::IntoVersionEnum for DisconnectV00765 {
	type Packet = super::super::Disconnect;

	fn into_version_enum(self) -> Self::Packet {
		super::super::Disconnect::V00765(self)
	}
}
impl crate::IntoPacketEnum for DisconnectV00765 {
	type State = super::super::super::Configuration;

	fn into_packet_enum(self) -> Self::State {
		let packet = crate::IntoVersionEnum::into_version_enum(self);
		super::super::super::Configuration::Disconnect(packet)
	}
}
impl crate::IntoStateEnum for DisconnectV00765 {
	type Direction = super::super::super::super::S2C;

	fn into_state_enum(self) -> Self::Direction {
		let state = crate::IntoPacketEnum::into_packet_enum(self);
		super::super::super::super::S2C::Configuration(state)
	}
}
