use crate::{datatypes::VarInt, protocol::S2C, Error, MinecraftProtocol, Result};
use status::{Pong, StatusResponse};
use std::io::Write;

/// Module for the Legacy server list ping
/// This state is stable and can be used with any protocol version
pub mod legacy;
/// Module containing all packets, structs and enums of the `Status` state.
/// This state is stable and can be used with any protocol version
pub mod status;

/// Enum containing all possible packets of the `Status` state.
#[derive(Debug, Clone, PartialEq)]
pub enum StatusPacket<'a> {
	StatusResponse { packet: StatusResponse<'a> },
	Pong { packet: Pong },
}

impl<'a> MinecraftProtocol<'a> for StatusPacket<'a> {
	fn read(protocol_version: u32, input: &'a [u8]) -> Result<(&'a [u8], Self)> {
		let (input, packet_id) = VarInt::read(protocol_version, input)?;

		match packet_id.0 {
			0x00 => {
				let (input, packet) = StatusResponse::read(protocol_version, input)?;
				Ok((input, StatusPacket::StatusResponse { packet }))
			}
			0x01 => {
				let (input, packet) = Pong::read(protocol_version, input)?;
				Ok((input, StatusPacket::Pong { packet }))
			}
			_ => {
				return Err(Error::InvalidData(format!(
					"Invalid packet ID: {}",
					packet_id.0
				)))
			}
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

impl<'a> Into<S2C<'a>> for StatusPacket<'a> {
	fn into(self) -> S2C<'a> {
		S2C::Status(self)
	}
}
