
#[allow(unused_imports)]
use craftflow_protocol_core::datatypes::*;
#[allow(unused_imports)]
use craftflow_protocol_core::*;

#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Eq, Ord)]
pub struct LoginStartV00764 {
	pub username: String,
	pub player_uuid: u128,
}

impl MCPWrite for LoginStartV00764 {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		let mut written_bytes = 0;

		written_bytes += self.username.write(output)?;
		written_bytes += self.player_uuid.write(output)?;

		Ok(written_bytes)
	}
}

impl MCPRead for LoginStartV00764 {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input, username) = String::read(input)?;
		let (input, player_uuid) = u128::read(input)?;

		Ok((
			input,
			Self {
				username,
				player_uuid,
			},
		))
	}
}

impl crate::IntoVersionEnum for LoginStartV00764 {
	type Packet = super::super::LoginStart;

	fn into_version_enum(self) -> Self::Packet {
		super::super::LoginStart::V00764(self)
	}
}
impl crate::IntoPacketEnum for LoginStartV00764 {
	type State = super::super::super::Login;

	fn into_packet_enum(self) -> Self::State {
		let packet = crate::IntoVersionEnum::into_version_enum(self);
		super::super::super::Login::LoginStart(packet)
	}
}
impl crate::IntoStateEnum for LoginStartV00764 {
	type Direction = super::super::super::super::C2S;

	fn into_state_enum(self) -> Self::Direction {
		let state = crate::IntoPacketEnum::into_packet_enum(self);
		super::super::super::super::C2S::Login(state)
	}
}
