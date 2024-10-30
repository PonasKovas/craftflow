#[allow(unused_imports)]
use crate::types::v00764::*;
#[allow(unused_imports)]
use craftflow_protocol_core::datatypes::*;
#[allow(unused_imports)]
use craftflow_protocol_core::*;
use std::borrow::Cow;

#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
pub struct CustomPayloadV00764<'a> {
	pub channel: Cow<'a, str>,
	pub data: RestBuffer<'a>,
}

impl<'a> MCPWrite for CustomPayloadV00764<'a> {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		let mut written_bytes = 0;

		written_bytes += self.channel.write(output)?;
		written_bytes += self.data.write(output)?;

		Ok(written_bytes)
	}
}

impl<'a> MCPRead<'a> for CustomPayloadV00764<'a> {
	fn read(input: &'a [u8]) -> Result<(&'a [u8], Self)> {
		let (input, channel) = Cow::read(input)?;
		let (input, data) = RestBuffer::read(input)?;

		Ok((input, Self { channel, data }))
	}
}

impl<'a> crate::IntoVersionEnum for CustomPayloadV00764<'a> {
	type Packet = super::super::CustomPayload<'a>;

	fn into_version_enum(self) -> Self::Packet {
		super::super::CustomPayload::V00764(self)
	}
}
impl<'a> crate::IntoPacketEnum for CustomPayloadV00764<'a> {
	type State = super::super::super::Configuration<'a>;

	fn into_packet_enum(self) -> Self::State {
		let packet = crate::IntoVersionEnum::into_version_enum(self);
		super::super::super::Configuration::CustomPayload(packet)
	}
}
impl<'a> crate::IntoStateEnum for CustomPayloadV00764<'a> {
	type Direction = super::super::super::super::C2S<'a>;

	fn into_state_enum(self) -> Self::Direction {
		let state = crate::IntoPacketEnum::into_packet_enum(self);
		super::super::super::super::C2S::Configuration(state)
	}
}
