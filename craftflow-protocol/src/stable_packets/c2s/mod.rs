use crate::{datatypes::VarInt, protocol::C2S, Error, MinecraftProtocol, Result};
use status::{Ping, StatusRequest};
use std::io::Write;

/// Module containing all packets, structs and enums of the `Handshake` state.
/// This state is stable and can be used with any protocol version
pub mod handshake;
/// Module for the Legacy server list ping
/// This state is stable and can be used with any protocol version
pub mod legacy;
/// Module containing all packets, structs and enums of the `Status` state.
/// This state is stable and can be used with any protocol version
pub mod status;

/// Enum containing all possible packets of the `Status` state.
#[derive(Debug, Clone, PartialEq)]
pub enum StatusPacket {
	StatusRequest { packet: StatusRequest },
	Ping { packet: Ping },
}

impl<'a> MinecraftProtocol<'a> for StatusPacket {
	fn read(protocol_version: u32, input: &[u8]) -> Result<(&[u8], Self)> {
		let (input, packet_id) = VarInt::read(protocol_version, input)?;

		match packet_id.0 {
			0x00 => {
				let (input, packet) = StatusRequest::read(protocol_version, input)?;

				Ok((input, StatusPacket::StatusRequest { packet }))
			}
			0x01 => {
				let (input, packet) = Ping::read(protocol_version, input)?;

				Ok((input, StatusPacket::Ping { packet }))
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
			StatusPacket::StatusRequest { packet } => {
				written += VarInt(0x00).write(protocol_version, output)?;
				written += packet.write(protocol_version, output)?;
			}
			StatusPacket::Ping { packet } => {
				written += VarInt(0x01).write(protocol_version, output)?;
				written += packet.write(protocol_version, output)?;
			}
		}

		Ok(written)
	}
}

impl<'a> Into<C2S<'a>> for StatusPacket {
	fn into(self) -> C2S<'a> {
		C2S::Status(self)
	}
}
