use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, State, WriteResult};
use anyhow::Result;
use craftflow_protocol_core::datatypes::Array;
use craftflow_protocol_versions::{
	c2s::{
		login::{
			login_start::{
				v00758::LoginStartV00005,
				v00759::{self, LoginStartV00759},
				v00760::{self, LoginStartV00760},
				v00763::LoginStartV00761,
				v00765::LoginStartV00764,
			},
			LoginStart,
		},
		Login,
	},
	IntoStateEnum, C2S,
};
use std::iter::{once, Once};

/// Starts the login process
#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbLoginStart {
	pub username: String,
	pub signature: Option<Signature>,
	pub uuid: Option<u128>,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct Signature {
	pub timestamp: i64,
	pub public_key: Vec<u8>,
	pub signature: Vec<u8>,
}

impl AbPacketWrite for AbLoginStart {
	type Direction = C2S;
	type Iter = Once<Self::Direction>;

	fn convert(self, protocol_version: u32, state: State) -> Result<WriteResult<Self::Iter>> {
		if state != State::Login {
			return Ok(WriteResult::Unsupported);
		}

		let pkt = match protocol_version {
			5..759 => LoginStartV00005 {
				username: self.username,
			}
			.into_state_enum(),
			759 => LoginStartV00759 {
				username: self.username,
				signature: self.signature.map(|s| v00759::Signature {
					timestamp: s.timestamp,
					public_key: Array::new(s.public_key),
					signature: Array::new(s.signature),
				}),
			}
			.into_state_enum(),
			760 => LoginStartV00760 {
				username: self.username,
				signature: self.signature.map(|s| v00760::Signature {
					timestamp: s.timestamp,
					public_key: Array::new(s.public_key),
					signature: Array::new(s.signature),
				}),
				player_uuid: self.uuid,
			}
			.into_state_enum(),
			761..764 => LoginStartV00761 {
				username: self.username,
				player_uuid: self.uuid,
			}
			.into_state_enum(),
			764.. => LoginStartV00764 {
				username: self.username,
				player_uuid: self.uuid.unwrap_or(0),
			}
			.into_state_enum(),
			_ => return Ok(WriteResult::Unsupported),
		};

		Ok(WriteResult::Success(once(pkt)))
	}
}

impl AbPacketNew for AbLoginStart {
	type Direction = C2S;
	type Constructor = NoConstructor<Self, C2S>;

	fn construct(
		packet: Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor, Self::Direction>> {
		Ok(match packet {
			C2S::Login(Login::LoginStart(pkt)) => match pkt {
				LoginStart::V00005(pkt) => ConstructorResult::Done(Self {
					username: pkt.username,
					signature: None,
					uuid: None,
				}),
				LoginStart::V00759(pkt) => ConstructorResult::Done(Self {
					username: pkt.username,
					signature: pkt.signature.map(|s| Signature {
						timestamp: s.timestamp,
						public_key: s.public_key.data,
						signature: s.signature.data,
					}),
					uuid: None,
				}),
				LoginStart::V00760(pkt) => ConstructorResult::Done(Self {
					username: pkt.username,
					signature: pkt.signature.map(|s| Signature {
						timestamp: s.timestamp,
						public_key: s.public_key.data,
						signature: s.signature.data,
					}),
					uuid: pkt.player_uuid,
				}),
				LoginStart::V00761(pkt) => ConstructorResult::Done(Self {
					username: pkt.username,
					signature: None,
					uuid: pkt.player_uuid,
				}),
				LoginStart::V00764(pkt) => ConstructorResult::Done(Self {
					username: pkt.username,
					signature: None,
					uuid: Some(pkt.player_uuid),
				}),
			},
			_ => ConstructorResult::Ignore(packet),
		})
	}
}
