#[allow(unused_imports)]
use craftflow_protocol_core::datatypes::*;
#[allow(unused_imports)]
use craftflow_protocol_core::*;

#[derive(Debug, PartialEq, Clone)]
pub struct EncryptionBeginV00759 {
	pub shared_secret: Buffer<VarInt>,
	pub crypto: Crypto,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Crypto {
	WithVerifyToken {
		verify_token: Buffer<VarInt>,
	},
	WithoutVerifyToken {
		salt: i64,
		message_signature: Buffer<VarInt>,
	},
}

impl MCPWrite for EncryptionBeginV00759 {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		let mut written_bytes = 0;

		written_bytes += self.shared_secret.write(output)?;
		written_bytes += self.crypto.write(output)?;

		Ok(written_bytes)
	}
}

impl MCPWrite for Crypto {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		let mut written_bytes = 0;

		match self {
			Crypto::WithVerifyToken { verify_token } => {
				written_bytes += true.write(output)?;
				written_bytes += verify_token.write(output)?;
			}
			Crypto::WithoutVerifyToken {
				salt,
				message_signature,
			} => {
				written_bytes += false.write(output)?;
				written_bytes += salt.write(output)?;
				written_bytes += message_signature.write(output)?;
			}
		}

		Ok(written_bytes)
	}
}

impl MCPRead for EncryptionBeginV00759 {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input, shared_secret) = Buffer::<VarInt>::read(input)?;
		let (input, crypto) = Crypto::read(input)?;

		Ok((
			input,
			Self {
				shared_secret,
				crypto,
			},
		))
	}
}

impl MCPRead for Crypto {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input, has_verify_token) = bool::read(input)?;
		if has_verify_token {
			let (input, verify_token) = Buffer::<VarInt>::read(input)?;
			Ok((input, Self::WithVerifyToken { verify_token }))
		} else {
			let (input, salt) = i64::read(input)?;
			let (input, message_signature) = Buffer::<VarInt>::read(input)?;
			Ok((
				input,
				Self::WithoutVerifyToken {
					salt,
					message_signature,
				},
			))
		}
	}
}

impl crate::IntoVersionEnum for EncryptionBeginV00759 {
	type Packet = super::super::EncryptionBegin;

	fn into_version_enum(self) -> Self::Packet {
		super::super::EncryptionBegin::V00759(self)
	}
}
impl crate::IntoPacketEnum for EncryptionBeginV00759 {
	type State = super::super::super::Login;

	fn into_packet_enum(self) -> Self::State {
		let packet = crate::IntoVersionEnum::into_version_enum(self);
		super::super::super::Login::EncryptionBegin(packet)
	}
}
impl crate::IntoStateEnum for EncryptionBeginV00759 {
	type Direction = super::super::super::super::C2S;

	fn into_state_enum(self) -> Self::Direction {
		let state = crate::IntoPacketEnum::into_packet_enum(self);
		super::super::super::super::C2S::Login(state)
	}
}
