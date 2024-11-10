use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, State, WriteResult};
use anyhow::{bail, Context, Result};
use craftflow_protocol_core::datatypes::Buffer;
use craftflow_protocol_versions::{
	c2s::{
		login::{
			encryption_begin::{
				v00005::EncryptionBeginV00005, v00047::EncryptionBeginV00047, v00759,
				v00759::EncryptionBeginV00759,
			},
			EncryptionBegin,
		},
		Login,
	},
	IntoStateEnum, C2S,
};
use shallowclone::ShallowClone;
use std::{
	array,
	borrow::Cow,
	iter::{once, Once},
};

#[derive(ShallowClone, Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbLoginEncryption<'a> {
	pub shared_secret: [u8; 16],
	pub verify_token: Option<Cow<'a, [u8]>>,
	pub signature: Option<Signature<'a>>,
}

#[derive(ShallowClone, Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct Signature<'a> {
	pub salt: i64,
	pub signature: Cow<'a, [u8]>,
}

impl<'a> AbPacketWrite<'a> for AbLoginEncryption<'a> {
	type Direction = C2S<'a>;
	type Iter = Once<Self::Direction>;

	fn convert(&'a self, protocol_version: u32, state: State) -> Result<WriteResult<Self::Iter>> {
		if state != State::Login {
			return Ok(WriteResult::Unsupported);
		}

		let pkt = match protocol_version {
			5..47 => EncryptionBeginV00005 {
				shared_secret: Buffer::from(&self.shared_secret[..]),
				verify_token: Buffer::from(
					self.verify_token
						.shallow_clone()
						.context("version requires verify_token")?,
				),
			}
			.into_state_enum(),
			47..759 => EncryptionBeginV00047 {
				shared_secret: Buffer::from(&self.shared_secret[..]),
				verify_token: Buffer::from(
					self.verify_token
						.shallow_clone()
						.context("version requires verify_token")?,
				),
			}
			.into_state_enum(),
			759.. => EncryptionBeginV00759 {
				shared_secret: Buffer::from(&self.shared_secret[..]),
				crypto: match &self.verify_token {
					Some(token) => v00759::Crypto::VerifyToken {
						verify_token: Buffer::from(token.shallow_clone()),
					},
					None => match &self.signature {
						Some(sig) => v00759::Crypto::SaltAndSignature {
							salt: sig.salt,
							message_signature: Buffer::from(sig.signature.shallow_clone()),
						},
						None => bail!("version requires verify_token or signature"),
					},
				},
			}
			.into_state_enum(),
			_ => return Ok(WriteResult::Unsupported),
		};

		Ok(WriteResult::Success(once(pkt)))
	}
}

fn secret_to_arr(secret: &Cow<[u8]>) -> Result<[u8; 16]> {
	if secret.len() != 16 {
		bail!("shared secret not 16 bytes");
	}

	Ok(array::from_fn(|i| secret[i]))
}

impl<'a> AbPacketNew<'a> for AbLoginEncryption<'a> {
	type Direction = C2S<'a>;
	type Constructor = NoConstructor<Self, C2S<'a>>;

	fn construct(
		packet: &'a Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor>> {
		Ok(match packet {
			C2S::Login(Login::EncryptionBegin(pkt)) => match pkt {
				EncryptionBegin::V00005(pkt) => ConstructorResult::Done(Self {
					shared_secret: secret_to_arr(&pkt.shared_secret.inner)?,
					verify_token: Some(pkt.verify_token.inner.shallow_clone()),
					signature: None,
				}),
				EncryptionBegin::V00047(pkt) => ConstructorResult::Done(Self {
					shared_secret: secret_to_arr(&pkt.shared_secret.inner)?,
					verify_token: Some(pkt.verify_token.inner.shallow_clone()),
					signature: None,
				}),
				EncryptionBegin::V00759(pkt) => {
					let verify_token;
					let signature;

					match &pkt.crypto {
						v00759::Crypto::VerifyToken {
							verify_token: token,
						} => {
							verify_token = Some(token.inner.shallow_clone());
							signature = None;
						}
						v00759::Crypto::SaltAndSignature {
							salt,
							message_signature,
						} => {
							verify_token = None;
							signature = Some(Signature {
								salt: *salt,
								signature: message_signature.inner.shallow_clone(),
							});
						}
					}
					ConstructorResult::Done(Self {
						shared_secret: secret_to_arr(&pkt.shared_secret.inner)?,
						verify_token,
						signature,
					})
				}
			},
			_ => ConstructorResult::Ignore,
		})
	}
}
