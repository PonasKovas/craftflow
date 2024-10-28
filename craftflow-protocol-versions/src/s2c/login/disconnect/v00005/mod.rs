use common_structures::Text;
#[allow(unused_imports)]
use craftflow_protocol_core::datatypes::*;
#[allow(unused_imports)]
use craftflow_protocol_core::*;

#[derive(Debug, PartialEq, Clone, Hash, PartialOrd)]
pub struct DisconnectV00005 {
	pub reason: Json<Text>,
}
impl MCPWrite for DisconnectV00005 {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		self.reason.write(output)
	}
}
impl MCPRead for DisconnectV00005 {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input, reason) = Json::<_>::read(input)?;
		Ok((input, Self { reason }))
	}
}

impl crate::IntoVersionEnum for DisconnectV00005 {
	type Packet = super::super::Disconnect;

	fn into_version_enum(self) -> Self::Packet {
		super::super::Disconnect::V00005(self)
	}
}
impl crate::IntoPacketEnum for DisconnectV00005 {
	type State = super::super::super::Login;

	fn into_packet_enum(self) -> Self::State {
		let packet = crate::IntoVersionEnum::into_version_enum(self);
		super::super::super::Login::Disconnect(packet)
	}
}
impl crate::IntoStateEnum for DisconnectV00005 {
	type Direction = super::super::super::super::S2C;

	fn into_state_enum(self) -> Self::Direction {
		let state = crate::IntoPacketEnum::into_packet_enum(self);
		super::super::super::super::S2C::Login(state)
	}
}
