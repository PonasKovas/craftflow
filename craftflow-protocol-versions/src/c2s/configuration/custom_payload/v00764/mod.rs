#[allow(unused_imports)]
use crate::types::v00764::*;
#[allow(unused_imports)]
use craftflow_protocol_core::datatypes::*;
#[allow(unused_imports)]
use craftflow_protocol_core::*;

#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
pub struct CustomPayloadV00764 {
	pub channel: String,
	pub data: RestBuffer,
}

impl MCPWrite for CustomPayloadV00764 {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		let mut written_bytes = 0;

		written_bytes += self.channel.write(output)?;
		written_bytes += self.data.write(output)?;

		Ok(written_bytes)
	}
}

impl MCPRead for CustomPayloadV00764 {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input, channel) = String::read(input)?;
		let (input, data) = RestBuffer::read(input)?;

		Ok((input, Self { channel, data }))
	}
}

impl crate::IntoVersionEnum for CustomPayloadV00764 {
	type Packet = super::super::CustomPayload;

	fn into_version_enum(self) -> Self::Packet {
		super::super::CustomPayload::V00764(self)
	}
}
impl crate::IntoPacketEnum for CustomPayloadV00764 {
	type State = super::super::super::Configuration;

	fn into_packet_enum(self) -> Self::State {
		let packet = crate::IntoVersionEnum::into_version_enum(self);
		super::super::super::Configuration::CustomPayload(packet)
	}
}
impl crate::IntoStateEnum for CustomPayloadV00764 {
	type Direction = super::super::super::super::C2S;

	fn into_state_enum(self) -> Self::Direction {
		let state = crate::IntoPacketEnum::into_packet_enum(self);
		super::super::super::super::C2S::Configuration(state)
	}
}
