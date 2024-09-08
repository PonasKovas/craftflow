use std::io::{Read, Write};

use crate::{datatypes::VarInt, protocol::C2S, MinecraftProtocol};
use anyhow::{bail, Result};
use status::{Ping, StatusRequest};

pub mod handshake;
pub mod legacy;
pub mod status;

#[derive(Debug, Clone, PartialEq)]
pub enum StatusPacket {
	StatusRequest { packet: StatusRequest },
	Ping { packet: Ping },
}

impl MinecraftProtocol for StatusPacket {
	fn read(protocol_version: u32, input: &mut impl Read) -> Result<Self> {
		let packet_id = VarInt::read(protocol_version, input)?;

		match packet_id.0 {
			0x00 => Ok(StatusPacket::StatusRequest {
				packet: StatusRequest::read(protocol_version, input)?,
			}),
			0x01 => Ok(StatusPacket::Ping {
				packet: Ping::read(protocol_version, input)?,
			}),
			_ => bail!("Unknown packet id: {}", packet_id.0),
		}
	}

	fn write(&self, protocol_version: u32, output: &mut impl Write) -> Result<usize> {
		match self {
			StatusPacket::StatusRequest { packet } => {
				VarInt(0x00).write(protocol_version, output)?;
				packet.write(protocol_version, output)
			}
			StatusPacket::Ping { packet } => {
				VarInt(0x01).write(protocol_version, output)?;
				packet.write(protocol_version, output)
			}
		}
	}
}

impl Into<C2S> for StatusPacket {
	fn into(self) -> C2S {
		C2S::Status(self)
	}
}
