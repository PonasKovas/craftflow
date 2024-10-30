use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, State, WriteResult};
use anyhow::Result;
use craftflow_protocol_core::datatypes::Array;
use craftflow_protocol_versions::{
	s2c::{
		login::{
			encryption_begin::{
				v00005::EncryptionBeginV00005, v00765::EncryptionBeginV00047,
				v00767::EncryptionBeginV00766,
			},
			EncryptionBegin,
		},
		Login,
	},
	IntoStateEnum, S2C,
};
use std::iter::{once, Once};

/// Initiates the encryption of the connection
#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbLoginEncryptionBegin {
	pub server_id: String,
	pub public_key: Vec<u8>,
	/// Any sequence of bytes, which will be sent back encrypted to verify that everything is correct.
	pub verify_token: Vec<u8>,
	pub should_authenticate: bool,
}

impl AbPacketWrite for AbLoginEncryptionBegin {
	type Direction = S2C;
	type Iter = Once<Self::Direction>;

	fn convert(self, protocol_version: u32, state: State) -> Result<WriteResult<Self::Iter>> {
		if state != State::Login {
			return Ok(WriteResult::Unsupported);
		}

		let pkt = match protocol_version {
			5..47 => EncryptionBeginV00005 {
				server_id: self.server_id,
				public_key: Array::new(self.public_key),
				verify_token: Array::new(self.verify_token),
			}
			.into_state_enum(),
			47..766 => EncryptionBeginV00047 {
				server_id: self.server_id,
				public_key: Array::new(self.public_key),
				verify_token: Array::new(self.verify_token),
			}
			.into_state_enum(),
			766.. => EncryptionBeginV00766 {
				server_id: self.server_id,
				public_key: Array::new(self.public_key),
				verify_token: Array::new(self.verify_token),
				should_authenticate: self.should_authenticate,
			}
			.into_state_enum(),
			_ => unimplemented!(),
		};

		Ok(WriteResult::Success(once(pkt)))
	}
}

impl AbPacketNew for AbLoginEncryptionBegin {
	type Direction = S2C;
	type Constructor = NoConstructor<Self, S2C>;

	fn construct(
		packet: Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor, Self::Direction>> {
		Ok(match packet {
			S2C::Login(Login::EncryptionBegin(pkt)) => match pkt {
				EncryptionBegin::V00005(pkt) => ConstructorResult::Done(Self {
					server_id: pkt.server_id,
					public_key: pkt.public_key.data,
					verify_token: pkt.verify_token.data,
					should_authenticate: true,
				}),
				EncryptionBegin::V00047(pkt) => ConstructorResult::Done(Self {
					server_id: pkt.server_id,
					public_key: pkt.public_key.data,
					verify_token: pkt.verify_token.data,
					should_authenticate: true,
				}),
				EncryptionBegin::V00766(pkt) => ConstructorResult::Done(Self {
					server_id: pkt.server_id,
					public_key: pkt.public_key.data,
					verify_token: pkt.verify_token.data,
					should_authenticate: pkt.should_authenticate,
				}),
			},
			_ => ConstructorResult::Ignore(packet),
		})
	}
}
