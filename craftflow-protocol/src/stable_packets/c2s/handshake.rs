use crate::{datatypes::VarInt, protocol::C2S, MinecraftProtocol, Packet};
use anyhow::bail;

#[derive(Debug, Clone, PartialEq)]
pub struct Handshake {
	pub protocol_version: VarInt,
	pub server_address: String,
	pub server_port: u16,
	pub next_state: NextState,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum NextState {
	Status = 1,
	Login = 2,
	Transfer = 3,
}

impl Packet for Handshake {
	type Direction = C2S;

	fn into_packet_enum(self) -> Self::Direction {
		C2S::Handshake(self)
	}
}

impl MinecraftProtocol for Handshake {
	fn read(protocol_version: u32, input: &mut impl std::io::Read) -> anyhow::Result<Self>
	where
		Self: Sized,
	{
		if VarInt::read(protocol_version, input)?.0 != 0x00 {
			bail!("Invalid packet ID for Handshake");
		}

		Ok(Self {
			protocol_version: MinecraftProtocol::read(protocol_version, input)?,
			server_address: MinecraftProtocol::read(protocol_version, input)?,
			server_port: MinecraftProtocol::read(protocol_version, input)?,
			next_state: match VarInt::read(protocol_version, input)?.0 {
				1 => NextState::Status,
				2 => NextState::Login,
				3 => NextState::Transfer,
				_ => bail!("Invalid NextState"),
			},
		})
	}

	fn write(
		&self,
		protocol_version: u32,
		output: &mut impl std::io::Write,
	) -> anyhow::Result<usize> {
		let mut written = 0;

		written += VarInt(0x00).write(protocol_version, output)?;
		written += self.protocol_version.write(protocol_version, output)?;
		written += self.server_address.write(protocol_version, output)?;
		written += self.server_port.write(protocol_version, output)?;
		written += VarInt(self.next_state as i32).write(protocol_version, output)?;

		Ok(written)
	}
}

impl Into<C2S> for Handshake {
	fn into(self) -> C2S {
		C2S::Handshake(self)
	}
}
