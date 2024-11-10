use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, State, WriteResult};
use anyhow::Result;
use craftflow_protocol_core::datatypes::Buffer;
use craftflow_protocol_versions::{
	c2s::{
		login::{
			login_start::{
				v00005::LoginStartV00005,
				v00759::{self, LoginStartV00759},
				v00760::{self, LoginStartV00760},
				v00761::LoginStartV00761,
				v00764::LoginStartV00764,
			},
			LoginStart,
		},
		Login,
	},
	IntoStateEnum, C2S,
};
use shallowclone::ShallowClone;
use std::{
	borrow::Cow,
	iter::{once, Once},
};

/// Starts the login process
#[derive(ShallowClone, Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbLoginStart<'a> {
	pub username: Cow<'a, str>,
	pub signature: Option<Signature<'a>>,
	pub uuid: Option<u128>,
}

#[derive(ShallowClone, Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct Signature<'a> {
	pub timestamp: i64,
	pub public_key: Cow<'a, [u8]>,
	pub signature: Cow<'a, [u8]>,
}

impl<'a> AbPacketWrite<'a> for AbLoginStart<'a> {
	type Direction = C2S<'a>;
	type Iter = Once<Self::Direction>;

	fn convert(&'a self, protocol_version: u32, state: State) -> Result<WriteResult<Self::Iter>> {
		if state != State::Login {
			return Ok(WriteResult::Unsupported);
		}

		let pkt = match protocol_version {
			5..759 => LoginStartV00005 {
				username: self.username.shallow_clone(),
			}
			.into_state_enum(),
			759 => LoginStartV00759 {
				username: self.username.shallow_clone(),
				signature: self.signature.shallow_clone().map(|s| v00759::Signature {
					timestamp: s.timestamp,
					public_key: Buffer::from(s.public_key),
					signature: Buffer::from(s.signature),
				}),
			}
			.into_state_enum(),
			760 => LoginStartV00760 {
				username: self.username.shallow_clone(),
				signature: self.signature.shallow_clone().map(|s| v00760::Signature {
					timestamp: s.timestamp,
					public_key: Buffer::from(s.public_key),
					signature: Buffer::from(s.signature),
				}),
				player_uuid: self.uuid,
			}
			.into_state_enum(),
			761..764 => LoginStartV00761 {
				username: self.username.shallow_clone(),
				player_uuid: self.uuid,
			}
			.into_state_enum(),
			764.. => LoginStartV00764 {
				username: self.username.shallow_clone(),
				player_uuid: self.uuid.unwrap_or(0),
			}
			.into_state_enum(),
			_ => return Ok(WriteResult::Unsupported),
		};

		Ok(WriteResult::Success(once(pkt)))
	}
}

impl<'a> AbPacketNew<'a> for AbLoginStart<'a> {
	type Direction = C2S<'a>;
	type Constructor = NoConstructor<Self, C2S<'a>>;

	fn construct(
		packet: &'a Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor>> {
		Ok(match packet {
			C2S::Login(Login::LoginStart(pkt)) => match pkt {
				LoginStart::V00005(pkt) => ConstructorResult::Done(Self {
					username: pkt.username.shallow_clone(),
					signature: None,
					uuid: None,
				}),
				LoginStart::V00759(pkt) => ConstructorResult::Done(Self {
					username: pkt.username.shallow_clone(),
					signature: pkt.signature.shallow_clone().map(|s| Signature {
						timestamp: s.timestamp,
						public_key: s.public_key.inner,
						signature: s.signature.inner,
					}),
					uuid: None,
				}),
				LoginStart::V00760(pkt) => ConstructorResult::Done(Self {
					username: pkt.username.shallow_clone(),
					signature: pkt.signature.shallow_clone().map(|s| Signature {
						timestamp: s.timestamp,
						public_key: s.public_key.inner,
						signature: s.signature.inner,
					}),
					uuid: pkt.player_uuid,
				}),
				LoginStart::V00761(pkt) => ConstructorResult::Done(Self {
					username: pkt.username.shallow_clone(),
					signature: None,
					uuid: pkt.player_uuid,
				}),
				LoginStart::V00764(pkt) => ConstructorResult::Done(Self {
					username: pkt.username.shallow_clone(),
					signature: None,
					uuid: Some(pkt.player_uuid),
				}),
			},
			_ => ConstructorResult::Ignore,
		})
	}
}
