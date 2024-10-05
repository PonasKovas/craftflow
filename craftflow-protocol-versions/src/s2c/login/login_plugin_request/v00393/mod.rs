#[allow(unused_imports)]
use craftflow_protocol_core::datatypes::*;
#[allow(unused_imports)]
use craftflow_protocol_core::*;

#[derive(Debug, PartialEq, Clone)]
pub struct LoginPluginRequestV00393 {
	pub message_id: VarInt,
	pub channel: String,
	pub data: RestBuffer,
}

impl MCPWrite for LoginPluginRequestV00393 {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		let mut written_bytes = 0;

		written_bytes += self.message_id.write(output)?;
		written_bytes += self.channel.write(output)?;
		written_bytes += self.data.write(output)?;

		Ok(written_bytes)
	}
}

impl MCPRead for LoginPluginRequestV00393 {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input, message_id) = VarInt::read(input)?;
		let (input, channel) = String::read(input)?;
		let (input, data) = RestBuffer::read(input)?;

		Ok((
			input,
			Self {
				message_id,
				channel,
				data,
			},
		))
	}
}

impl crate::IntoVersionEnum for LoginPluginRequestV00393 {
	type Packet = super::super::LoginPluginRequest;

	fn into_version_enum(self) -> Self::Packet {
		super::super::LoginPluginRequest::V00393(self)
	}
}
impl crate::IntoPacketEnum for LoginPluginRequestV00393 {
	type State = super::super::super::Login;

	fn into_packet_enum(self) -> Self::State {
		let packet = crate::IntoVersionEnum::into_version_enum(self);
		super::super::super::Login::LoginPluginRequest(packet)
	}
}
impl crate::IntoStateEnum for LoginPluginRequestV00393 {
	type Direction = super::super::super::super::S2C;

	fn into_state_enum(self) -> Self::Direction {
		let state = crate::IntoPacketEnum::into_packet_enum(self);
		super::super::super::super::S2C::Login(state)
	}
}
