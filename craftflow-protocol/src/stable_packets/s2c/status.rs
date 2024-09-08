use crate::{protocol::S2C, MinecraftProtocol, Packet};
use anyhow::Result;
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq)]
pub struct StatusResponse {
	pub json_response: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pong {
	pub payload: u64,
}

impl Packet for StatusResponse {
	type Direction = S2C;

	fn into_packet_enum(self) -> Self::Direction {
		S2C::Status(super::StatusPacket::StatusResponse { packet: self })
	}
}

impl Packet for Pong {
	type Direction = S2C;

	fn into_packet_enum(self) -> Self::Direction {
		S2C::Status(super::StatusPacket::Pong { packet: self })
	}
}

impl MinecraftProtocol for StatusResponse {
	fn read(protocol_version: u32, input: &mut impl Read) -> Result<Self>
	where
		Self: Sized,
	{
		Ok(Self {
			json_response: String::read(protocol_version, input)?,
		})
	}

	fn write(&self, protocol_version: u32, output: &mut impl Write) -> Result<usize> {
		let mut written = 0;

		written += self.json_response.write(protocol_version, output)?;

		Ok(written)
	}
}

impl MinecraftProtocol for Pong {
	fn read(protocol_version: u32, input: &mut impl Read) -> Result<Self> {
		Ok(Self {
			payload: u64::read(protocol_version, input)?,
		})
	}

	fn write(&self, protocol_version: u32, output: &mut impl Write) -> anyhow::Result<usize> {
		let mut written = 0;

		written += self.payload.write(protocol_version, output)?;

		Ok(written)
	}
}
