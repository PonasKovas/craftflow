use crate::{protocol::C2S, MinecraftProtocol, Packet, Result};
use std::io::Write;

#[derive(Debug, Clone, PartialEq)]
pub struct StatusRequest {}

#[derive(Debug, Clone, PartialEq)]
pub struct Ping {
	pub payload: u64,
}

impl Packet for StatusRequest {
	type Direction = C2S<'static>;
	type StaticSelf = StatusRequest;

	fn into_packet_enum(self) -> Self::Direction {
		C2S::Status(super::StatusPacket::StatusRequest { packet: self })
	}
}

impl Packet for Ping {
	type Direction = C2S<'static>;
	type StaticSelf = Ping;

	fn into_packet_enum(self) -> Self::Direction {
		C2S::Status(super::StatusPacket::Ping { packet: self })
	}
}

impl<'a> MinecraftProtocol<'a> for StatusRequest {
	fn read(_protocol_version: u32, input: &[u8]) -> Result<(&[u8], Self)> {
		Ok((input, Self {}))
	}

	fn write(&self, _protocol_version: u32, _output: &mut impl Write) -> Result<usize> {
		Ok(0)
	}
}

impl<'a> MinecraftProtocol<'a> for Ping {
	fn read(protocol_version: u32, input: &[u8]) -> Result<(&[u8], Self)> {
		let (input, payload) = u64::read(protocol_version, input)?;

		Ok((input, Self { payload }))
	}

	fn write(&self, protocol_version: u32, output: &mut impl Write) -> Result<usize> {
		let mut written = 0;

		written += self.payload.write(protocol_version, output)?;

		Ok(written)
	}
}
