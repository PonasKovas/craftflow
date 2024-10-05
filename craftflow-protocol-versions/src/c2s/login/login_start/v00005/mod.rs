#[allow(unused_imports)]
use craftflow_protocol_core::datatypes::*;
#[allow(unused_imports)]
use craftflow_protocol_core::*;

#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Eq, Ord)]
pub struct LoginStartV00005 {
	pub username: String,
}

impl MCPWrite for LoginStartV00005 {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		self.username.write(output)
	}
}

impl MCPRead for LoginStartV00005 {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input, username) = String::read(input)?;
		Ok((input, Self { username }))
	}
}

impl crate::IntoVersionEnum for LoginStartV00005 {
	type Packet = super::super::LoginStart;

	fn into_version_enum(self) -> Self::Packet {
		super::super::LoginStart::V00005(self)
	}
}
impl crate::IntoPacketEnum for LoginStartV00005 {
	type State = super::super::super::Login;

	fn into_packet_enum(self) -> Self::State {
		let packet = crate::IntoVersionEnum::into_version_enum(self);
		super::super::super::Login::LoginStart(packet)
	}
}
impl crate::IntoStateEnum for LoginStartV00005 {
	type Direction = super::super::super::super::C2S;

	fn into_state_enum(self) -> Self::Direction {
		let state = crate::IntoPacketEnum::into_packet_enum(self);
		super::super::super::super::C2S::Login(state)
	}
}
