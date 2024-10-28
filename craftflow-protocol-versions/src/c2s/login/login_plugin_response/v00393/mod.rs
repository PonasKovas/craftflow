#[allow(unused_imports)]
use craftflow_protocol_core::common_structures::*;
#[allow(unused_imports)]
use craftflow_protocol_core::datatypes::*;
#[allow(unused_imports)]
use craftflow_protocol_core::*;

#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
pub struct LoginPluginResponseV00393 {
	pub message_id: VarInt,
	pub data: Option<RestBuffer>,
}

impl MCPWrite for LoginPluginResponseV00393 {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		let mut written_bytes = 0;

		written_bytes += self.message_id.write(output)?;
		written_bytes += self.data.write(output)?;

		Ok(written_bytes)
	}
}

impl MCPRead for LoginPluginResponseV00393 {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input, message_id) = VarInt::read(input)?;
		let (input, data) = Option::<RestBuffer>::read(input)?;

		Ok((input, Self { message_id, data }))
	}
}

impl crate::IntoVersionEnum for LoginPluginResponseV00393 {
	type Packet = super::super::LoginPluginResponse;

	fn into_version_enum(self) -> Self::Packet {
		super::super::LoginPluginResponse::V00393(self)
	}
}
impl crate::IntoPacketEnum for LoginPluginResponseV00393 {
	type State = super::super::super::Login;

	fn into_packet_enum(self) -> Self::State {
		let packet = crate::IntoVersionEnum::into_version_enum(self);
		super::super::super::Login::LoginPluginResponse(packet)
	}
}
impl crate::IntoStateEnum for LoginPluginResponseV00393 {
	type Direction = super::super::super::super::C2S;

	fn into_state_enum(self) -> Self::Direction {
		let state = crate::IntoPacketEnum::into_packet_enum(self);
		super::super::super::super::C2S::Login(state)
	}
}
