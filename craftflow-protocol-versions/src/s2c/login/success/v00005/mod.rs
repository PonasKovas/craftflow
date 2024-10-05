#[allow(unused_imports)]
use craftflow_protocol_core::datatypes::*;
#[allow(unused_imports)]
use craftflow_protocol_core::*;

#[derive(Debug, PartialEq, Clone, Hash, PartialOrd)]
pub struct SuccessV00005 {
	pub uuid: String,
	pub username: String,
}
impl MCPWrite for SuccessV00005 {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		let mut written_bytes = 0;
		written_bytes += self.uuid.write(output)?;
		written_bytes += self.username.write(output)?;
		Ok(written_bytes)
	}
}
impl MCPRead for SuccessV00005 {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input, uuid) = String::read(input)?;
		let (input, username) = String::read(input)?;
		Ok((input, Self { uuid, username }))
	}
}

impl crate::IntoVersionEnum for SuccessV00005 {
	type Packet = super::super::Success;

	fn into_version_enum(self) -> Self::Packet {
		super::super::Success::V00005(self)
	}
}
impl crate::IntoPacketEnum for SuccessV00005 {
	type State = super::super::super::Login;

	fn into_packet_enum(self) -> Self::State {
		let packet = crate::IntoVersionEnum::into_version_enum(self);
		super::super::super::Login::Success(packet)
	}
}
impl crate::IntoStateEnum for SuccessV00005 {
	type Direction = super::super::super::super::S2C;

	fn into_state_enum(self) -> Self::Direction {
		let state = crate::IntoPacketEnum::into_packet_enum(self);
		super::super::super::super::S2C::Login(state)
	}
}
