use anyhow::{bail, Result};
use status::{Pong, StatusResponse};
use std::io::{Read, Write};

use crate::{datatypes::VarInt, protocol::S2C, MinecraftProtocol};

pub mod legacy;
pub mod status;

#[derive(Debug, Clone, PartialEq)]
pub enum StatusPacket {
	StatusResponse { packet: StatusResponse },
	Pong { packet: Pong },
}

impl MinecraftProtocol for StatusPacket {
	fn read(protocol_version: u32, input: &mut impl Read) -> Result<Self> {
		let packet_id = VarInt::read(protocol_version, input)?;

		match packet_id.0 {
			0x00 => Ok(StatusPacket::StatusResponse {
				packet: StatusResponse::read(protocol_version, input)?,
			}),
			0x01 => Ok(StatusPacket::Pong {
				packet: Pong::read(protocol_version, input)?,
			}),
			_ => bail!("Unknown packet id: {}", packet_id.0),
		}
	}

	fn write(&self, protocol_version: u32, output: &mut impl Write) -> Result<usize> {
		let mut written = 0;

		match self {
			StatusPacket::StatusResponse { packet } => {
				written += VarInt(0x00).write(protocol_version, output)?;
				written += packet.write(protocol_version, output)?;
			}
			StatusPacket::Pong { packet } => {
				written += VarInt(0x01).write(protocol_version, output)?;
				written += packet.write(protocol_version, output)?;
			}
		}

		Ok(written)
	}
}

impl Into<S2C> for StatusPacket {
	fn into(self) -> S2C {
		S2C::Status(self)
	}
}
