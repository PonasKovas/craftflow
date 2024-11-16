use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, State, WriteResult};
use anyhow::Result;
use craftflow_protocol_core::datatypes::Buffer;
use craftflow_protocol_versions::{
	s2c::{
		login::{
			encryption_begin::{
				v00005::EncryptionBeginV00005, v00047::EncryptionBeginV00047,
				v00766::EncryptionBeginV00766,
			},
			EncryptionBegin,
		},
		Login,
	},
	IntoStateEnum, S2C,
};
use shallowclone::{MakeOwned, ShallowClone};
use std::{
	borrow::Cow,
	iter::{once, Once},
};

/// Initiates the encryption of the connection
#[derive(ShallowClone, MakeOwned, Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbLoginEncryptionBegin<'a> {
	pub server_id: Cow<'a, str>,
	pub public_key: Cow<'a, [u8]>,
	/// Any sequence of bytes, which will be sent back encrypted to verify that everything is correct.
	pub verify_token: Cow<'a, [u8]>,
	pub should_authenticate: bool,
}

impl<'a> AbPacketWrite<'a> for AbLoginEncryptionBegin<'a> {
	type Direction = S2C<'a>;
	type Iter = Once<Self::Direction>;

	fn convert(&'a self, protocol_version: u32, state: State) -> Result<WriteResult<Self::Iter>> {
		if state != State::Login {
			return Ok(WriteResult::Unsupported);
		}

		let pkt = match protocol_version {
			5..47 => EncryptionBeginV00005 {
				server_id: self.server_id.shallow_clone(),
				public_key: Buffer::from(self.public_key.shallow_clone()),
				verify_token: Buffer::from(self.verify_token.shallow_clone()),
			}
			.into_state_enum(),
			47..766 => EncryptionBeginV00047 {
				server_id: self.server_id.shallow_clone(),
				public_key: Buffer::from(self.public_key.shallow_clone()),
				verify_token: Buffer::from(self.verify_token.shallow_clone()),
			}
			.into_state_enum(),
			766.. => EncryptionBeginV00766 {
				server_id: self.server_id.shallow_clone(),
				public_key: Buffer::from(self.public_key.shallow_clone()),
				verify_token: Buffer::from(self.verify_token.shallow_clone()),
				should_authenticate: self.should_authenticate,
			}
			.into_state_enum(),
			_ => unimplemented!(),
		};

		Ok(WriteResult::Success(once(pkt)))
	}
}

impl<'a> AbPacketNew<'a> for AbLoginEncryptionBegin<'a> {
	type Direction = S2C<'a>;
	type Constructor = NoConstructor<AbLoginEncryptionBegin<'static>>;

	fn construct(
		packet: &'a Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor>> {
		Ok(match packet {
			S2C::Login(Login::EncryptionBegin(pkt)) => match pkt {
				EncryptionBegin::V00005(pkt) => ConstructorResult::Done(Self {
					server_id: pkt.server_id.shallow_clone(),
					public_key: pkt.public_key.inner.shallow_clone(),
					verify_token: pkt.verify_token.inner.shallow_clone(),
					should_authenticate: true,
				}),
				EncryptionBegin::V00047(pkt) => ConstructorResult::Done(Self {
					server_id: pkt.server_id.shallow_clone(),
					public_key: pkt.public_key.inner.shallow_clone(),
					verify_token: pkt.verify_token.inner.shallow_clone(),
					should_authenticate: true,
				}),
				EncryptionBegin::V00766(pkt) => ConstructorResult::Done(Self {
					server_id: pkt.server_id.shallow_clone(),
					public_key: pkt.public_key.inner.shallow_clone(),
					verify_token: pkt.verify_token.inner.shallow_clone(),
					should_authenticate: pkt.should_authenticate,
				}),
			},
			_ => ConstructorResult::Ignore,
		})
	}
}
