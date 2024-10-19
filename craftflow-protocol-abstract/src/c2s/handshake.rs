use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, WriteResult};
use anyhow::{bail, Result};
use craftflow_protocol_core::datatypes::VarInt;
use craftflow_protocol_versions::{
	c2s::{
		handshaking::{set_protocol::v00005::SetProtocolV00005, SetProtocol},
		Handshaking,
	},
	IntoStateEnum, C2S,
};
use std::iter::{once, Once};

/// The initial packet that a client should send to the server.
#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbHandshake {
	/// The protocol version that the client is using
	pub protocol_version: u32,
	/// The address that the client is connecting to
	pub address: String,
	/// The port that the client is connecting to
	pub port: u16,
	/// The next state that the client wants to switch to
	pub next_state: NextState,
}

/// The next state that the client wants to switch to
#[derive(Debug, PartialEq, Clone, Copy, Hash, PartialOrd, Eq, Ord)]
pub enum NextState {
	Status,
	Login,
	/// This is sent when the client is being transferred here from another server
	/// Only available since 1.20.5 version. Will be replaced with [`NextState::Login`] in older versions.
	Transfer,
}

impl AbPacketWrite for AbHandshake {
	type Direction = C2S;
	type Iter = Once<Self::Direction>;

	fn convert(self, protocol_version: u32) -> Result<WriteResult<Self::Iter>> {
		// The Handshake packet is identical in all protocol versions
		Ok(WriteResult::Success(once(
			SetProtocolV00005 {
				protocol_version: VarInt(self.protocol_version as i32),
				server_host: self.address,
				server_port: self.port,
				next_state: VarInt(match self.next_state {
					NextState::Status => 1,
					NextState::Login => 2,
					NextState::Transfer => {
						if protocol_version >= 766 {
							3
						} else {
							2
						}
					}
				}),
			}
			.into_state_enum(),
		)))
	}
}

impl AbPacketNew for AbHandshake {
	type Direction = C2S;
	type Constructor = NoConstructor<Self, C2S>;

	fn construct(
		packet: Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor, Self::Direction>> {
		match packet {
			C2S::Handshaking(Handshaking::SetProtocol(SetProtocol::V00005(packet))) => {
				Ok(ConstructorResult::Done(Self {
					protocol_version: packet.protocol_version.0 as u32,
					address: packet.server_host,
					port: packet.server_port,
					next_state: match packet.next_state.0 {
						1 => NextState::Status,
						2 => NextState::Login,
						3 => NextState::Transfer,
						_ => {
							bail!("Invalid next state {}", packet.next_state.0)
						}
					},
				}))
			}
			_ => Ok(ConstructorResult::Ignore(packet)),
		}
	}
}
