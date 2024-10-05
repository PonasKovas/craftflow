use craftflow_protocol_core::*;

#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
pub struct ServerInfoV00005 {
	pub response: String,
}
impl MCPWrite for ServerInfoV00005 {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		self.response.write(output)
	}
}
impl MCPRead for ServerInfoV00005 {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input, response) = String::read(input)?;
		Ok((input, Self { response }))
	}
}

impl crate::IntoVersionEnum for ServerInfoV00005 {
	type Packet = super::super::ServerInfo;

	fn into_version_enum(self) -> Self::Packet {
		super::super::ServerInfo::V00005(self)
	}
}
impl crate::IntoPacketEnum for ServerInfoV00005 {
	type State = super::super::super::Status;

	fn into_packet_enum(self) -> Self::State {
		let packet = crate::IntoVersionEnum::into_version_enum(self);
		super::super::super::Status::ServerInfo(packet)
	}
}
impl crate::IntoStateEnum for ServerInfoV00005 {
	type Direction = super::super::super::super::S2C;

	fn into_state_enum(self) -> Self::Direction {
		let state = crate::IntoPacketEnum::into_packet_enum(self);
		super::super::super::super::S2C::Status(state)
	}
}
