use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, State, WriteResult};
use anyhow::{bail, Context, Result};
use craftflow_protocol_core::datatypes::Array;
use craftflow_protocol_versions::{
	c2s::{
		login::{
			encryption_begin::{
				v00005::EncryptionBeginV00005, v00759, v00760::EncryptionBeginV00759,
				v00765::EncryptionBeginV00047,
			},
			EncryptionBegin,
		},
		Login,
	},
	IntoStateEnum, C2S,
};
use std::iter::{once, Once};

#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbLoginEncryption {
	pub shared_secret: Vec<u8>,
	pub verify_token: Option<Vec<u8>>,
	pub signature: Option<Signature>,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct Signature {
	pub salt: i64,
	pub signature: Vec<u8>,
}

impl AbPacketWrite for AbLoginEncryption {
	type Direction = C2S;
	type Iter = Once<Self::Direction>;

	fn convert(self, protocol_version: u32, state: State) -> Result<WriteResult<Self::Iter>> {
		if state != State::Login {
			return Ok(WriteResult::Unsupported);
		}

		let pkt = match protocol_version {
			5..47 => EncryptionBeginV00005 {
				shared_secret: Array::new(self.shared_secret),
				verify_token: Array::new(
					self.verify_token.context("version requires verify_token")?,
				),
			}
			.into_state_enum(),
			47..759 => EncryptionBeginV00047 {
				shared_secret: Array::new(self.shared_secret),
				verify_token: Array::new(
					self.verify_token.context("version requires verify_token")?,
				),
			}
			.into_state_enum(),
			759.. => EncryptionBeginV00759 {
				shared_secret: Array::new(self.shared_secret),
				crypto: match self.verify_token {
					Some(token) => v00759::Crypto::WithVerifyToken {
						verify_token: Array::new(token),
					},
					None => match self.signature {
						Some(sig) => v00759::Crypto::WithoutVerifyToken {
							salt: sig.salt,
							message_signature: Array::new(sig.signature),
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

impl AbPacketNew for AbLoginEncryption {
	type Direction = C2S;
	type Constructor = NoConstructor<Self, C2S>;

	fn construct(
		packet: Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor, Self::Direction>> {
		Ok(match packet {
			C2S::Login(Login::EncryptionBegin(pkt)) => match pkt {
				EncryptionBegin::V00005(pkt) => ConstructorResult::Done(Self {
					shared_secret: pkt.shared_secret.data,
					verify_token: Some(pkt.verify_token.data),
					signature: None,
				}),
				EncryptionBegin::V00047(pkt) => ConstructorResult::Done(Self {
					shared_secret: pkt.shared_secret.data,
					verify_token: Some(pkt.verify_token.data),
					signature: None,
				}),
				EncryptionBegin::V00759(pkt) => {
					let verify_token;
					let signature;
					match pkt.crypto {
						v00759::Crypto::WithVerifyToken {
							verify_token: token,
						} => {
							verify_token = Some(token.data);
							signature = None;
						}
						v00759::Crypto::WithoutVerifyToken {
							salt,
							message_signature,
						} => {
							verify_token = None;
							signature = Some(Signature {
								salt,
								signature: message_signature.data,
							});
						}
					}
					ConstructorResult::Done(Self {
						shared_secret: pkt.shared_secret.data,
						verify_token,
						signature,
					})
				}
			},
			_ => ConstructorResult::Ignore(packet),
		})
	}
}
