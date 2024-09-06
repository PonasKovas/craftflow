use crate::{datatypes::VarInt, MinecraftProtocol, Packet};
use anyhow::bail;
use std::io::{Read, Write};

/// The first packet of the protocol
#[derive(Debug, Clone, PartialEq)]
pub struct Handshake {
	pub protocol_version: i32,
	pub server_address: String,
	pub server_port: u16,
	pub next_state: NextState,
}

/// Describes the type of connection
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NextState {
	Status = 1,
	Login = 2,
}

impl MinecraftProtocol for Handshake {
	fn read(protocol_version: u32, source: &mut impl Read) -> anyhow::Result<Self> {
		Ok(Self {
			protocol_version: VarInt::read(protocol_version, source)?.0,
			server_address: String::read(protocol_version, source)?,
			server_port: u16::read(protocol_version, source)?,
			next_state: match u8::read(protocol_version, source)? {
				1 => NextState::Status,
				2 => NextState::Login,
				other => bail!("unknown next_state {other}"),
			},
		})
	}

	fn write(&self, protocol_version: u32, to: &mut impl Write) -> anyhow::Result<usize> {
		let mut written = 0;

		written += VarInt(self.protocol_version).write(protocol_version, to)?;
		written += self.server_address.write(protocol_version, to)?;
		written += self.server_port.write(protocol_version, to)?;
		written += (self.next_state as u8).write(protocol_version, to)?;

		Ok(written)
	}
}

impl Packet for Handshake {
	type Direction = crate::protocol::C2S;

	fn into_packet_enum(self) -> Self::Direction {
		crate::protocol::C2S::Handshake(self)
	}
}
