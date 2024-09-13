use crate::{datatypes::VarInt, protocol::C2S, Error, MinecraftProtocol, Packet, Result};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq)]
pub struct Handshake<'a> {
	pub protocol_version: VarInt,
	pub server_address: Cow<'a, str>,
	pub server_port: u16,
	pub next_state: NextState,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum NextState {
	Status = 1,
	Login = 2,
	Transfer = 3,
}

impl<'a> Packet for Handshake<'a> {
	type Direction = C2S<'a>;
	type StaticSelf = Handshake<'static>;

	fn into_packet_enum(self) -> Self::Direction {
		C2S::Handshake(self)
	}
}

impl<'a> MinecraftProtocol<'a> for Handshake<'a> {
	fn read(protocol_version: u32, input: &'a [u8]) -> Result<(&'a [u8], Self)> {
		let (input, packet_id) = VarInt::read(protocol_version, input)?;

		if packet_id.0 != 0x00 {
			return Err(Error::InvalidData(format!(
				"Invalid packet ID for Handshake: {}",
				packet_id.0
			)));
		}

		let (input, version) = MinecraftProtocol::read(protocol_version, input)?;
		let (input, server_address) = MinecraftProtocol::read(protocol_version, input)?;
		let (input, server_port) = MinecraftProtocol::read(protocol_version, input)?;
		let (input, next_state) = VarInt::read(protocol_version, input)?;

		Ok((
			input,
			Self {
				protocol_version: version,
				server_address,
				server_port,
				next_state: match next_state.0 {
					1 => NextState::Status,
					2 => NextState::Login,
					3 => NextState::Transfer,
					_ => {
						return Err(Error::InvalidData(format!(
							"Invalid NextState {}",
							next_state.0
						)))
					}
				},
			},
		))
	}

	fn write(&self, protocol_version: u32, output: &mut impl std::io::Write) -> Result<usize> {
		let mut written = 0;

		written += VarInt(0x00).write(protocol_version, output)?;
		written += self.protocol_version.write(protocol_version, output)?;
		written += self.server_address.write(protocol_version, output)?;
		written += self.server_port.write(protocol_version, output)?;
		written += VarInt(self.next_state as i32).write(protocol_version, output)?;

		Ok(written)
	}
}

impl<'a> Into<C2S<'a>> for Handshake<'a> {
	fn into(self) -> C2S<'a> {
		C2S::Handshake(self)
	}
}
