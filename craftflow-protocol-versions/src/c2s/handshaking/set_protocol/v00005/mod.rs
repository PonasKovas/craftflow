
use craftflow_protocol_core::datatypes::*;
use craftflow_protocol_core::*;

#[derive(Debug, PartialEq)]
pub struct SetProtocolV00005 {
	pub protocol_version: VarInt,
	pub server_host: String,
	pub server_port: u16,
	pub next_state: VarInt,
}
impl MCPWrite for SetProtocolV00005 {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		let mut written_bytes = 0;

		written_bytes += self.protocol_version.write(output)?;
		written_bytes += self.server_host.write(output)?;
		written_bytes += self.server_port.write(output)?;
		written_bytes += self.next_state.write(output)?;

		Ok(written_bytes)
	}
}
impl MCPRead for SetProtocolV00005 {
	fn read(input: &[u8]) -> Result<(&[u8], Self)> {
		let (input, protocol_version) = VarInt::read(input)?;
		let (input, server_host) = String::read(input)?;
		let (input, server_port) = u16::read(input)?;
		let (input, next_state) = VarInt::read(input)?;

		Ok((
			input,
			Self {
				protocol_version,
				server_host,
				server_port,
				next_state,
			},
		))
	}
}

impl crate::IntoVersionEnum for SetProtocolV00005 {
	type Packet = super::super::SetProtocol;

	fn into_version_enum(self) -> Self::Packet {
		super::super::SetProtocol::V00005(self)
	}
}
impl crate::IntoPacketEnum for SetProtocolV00005 {
	type State = super::super::super::Handshaking;

	fn into_packet_enum(self) -> Self::State {
		let packet = crate::IntoVersionEnum::into_version_enum(self);
		super::super::super::Handshaking::SetProtocol(packet)
	}
}
impl crate::IntoStateEnum for SetProtocolV00005 {
	type Direction = super::super::super::super::C2S;

	fn into_state_enum(self) -> Self::Direction {
		let state = crate::IntoPacketEnum::into_packet_enum(self);
		super::super::super::super::C2S::Handshaking(state)
	}
}
