use status_response::StatusResponseInner;

use crate::datatypes::Json;
use crate::{protocol::S2C, MinecraftProtocol, Packet, Result};
use std::io::Write;

pub mod status_response;

#[derive(Debug, Clone, PartialEq)]
pub struct StatusResponse<'a> {
	pub json_response: Json<StatusResponseInner<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pong {
	pub payload: u64,
}

impl<'a> Packet for StatusResponse<'a> {
	type Direction = S2C<'a>;
	type StaticSelf = StatusResponse<'static>;

	fn into_packet_enum(self) -> Self::Direction {
		S2C::Status(super::StatusPacket::StatusResponse { packet: self })
	}
}

impl Packet for Pong {
	type Direction = S2C<'static>;
	type StaticSelf = Pong;

	fn into_packet_enum(self) -> Self::Direction {
		S2C::Status(super::StatusPacket::Pong { packet: self })
	}
}

impl<'a> MinecraftProtocol<'a> for StatusResponse<'a> {
	fn read(protocol_version: u32, input: &[u8]) -> Result<(&[u8], Self)> {
		let (input, json_response) = Json::read(protocol_version, input)?;

		Ok((input, Self { json_response }))
	}

	fn write(&self, protocol_version: u32, output: &mut impl Write) -> Result<usize> {
		let mut written = 0;

		written += self.json_response.write(protocol_version, output)?;

		Ok(written)
	}
}

impl<'a> MinecraftProtocol<'a> for Pong {
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
