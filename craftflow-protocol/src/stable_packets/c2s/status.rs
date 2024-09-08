use crate::{protocol::C2S, MinecraftProtocol, Packet};
use anyhow::Result;
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq)]
pub struct StatusRequest {}

#[derive(Debug, Clone, PartialEq)]
pub struct Ping {
	pub payload: u64,
}

impl Packet for StatusRequest {
	type Direction = C2S;

	fn into_packet_enum(self) -> Self::Direction {
		C2S::Status(super::StatusPacket::StatusRequest { packet: self })
	}
}

impl Packet for Ping {
	type Direction = C2S;

	fn into_packet_enum(self) -> Self::Direction {
		C2S::Status(super::StatusPacket::Ping { packet: self })
	}
}

impl MinecraftProtocol for StatusRequest {
	fn read(_protocol_version: u32, _input: &mut impl Read) -> Result<Self> {
		Ok(Self {})
	}

	fn write(&self, _protocol_version: u32, _output: &mut impl Write) -> anyhow::Result<usize> {
		Ok(0)
	}
}

impl MinecraftProtocol for Ping {
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
