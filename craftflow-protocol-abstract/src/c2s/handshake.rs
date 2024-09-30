use crate::{AbConstrResult, AbPacketNew, AbPacketWrite};
use craftflow_protocol_core::{datatypes::VarInt, Error, Result};
use craftflow_protocol_versions::{
	c2s::{
		handshaking::{set_protocol::v00005::SetProtocolV00005, SetProtocol},
		Handshaking,
	},
	IntoStateEnum, C2S,
};

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
pub enum NextState {
	Status,
	Login,
	/// This is sent when the client is being transferred here from another server
	/// Only available since 1.20.5 version. If unsure of server version, use `Login` instead.
	Transfer,
}

impl AbPacketWrite for AbHandshake {
	type Direction = C2S;

	fn convert_and_write(
		self,
		protocol_version: u32,
		mut writer: impl FnMut(Self::Direction) -> Result<()>,
	) -> Result<()> {
		// The Handshake packet is identical in all protocol versions
		writer(
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
		)
	}
}

impl AbPacketNew for AbHandshake {
	type Direction = C2S;
	type Constructor = ();

	fn construct(packet: Self::Direction) -> Result<AbConstrResult<Self, (), Self::Direction>> {
		match packet {
			C2S::Handshaking(Handshaking::SetProtocol(SetProtocol::V00005(packet))) => {
				Ok(AbConstrResult::Done(Self {
					protocol_version: packet.protocol_version.0 as u32,
					address: packet.server_host,
					port: packet.server_port,
					next_state: match packet.next_state.0 {
						1 => NextState::Status,
						2 => NextState::Login,
						3 => NextState::Transfer,
						_ => {
							return Err(Error::InvalidData(format!(
								"Invalid next state {}",
								packet.next_state.0
							)))
						}
					},
				}))
			}
			_ => Ok(AbConstrResult::Ignore(((), packet))),
		}
	}
}
