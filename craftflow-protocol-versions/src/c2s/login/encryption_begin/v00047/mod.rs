#[allow(unused_imports)]
use craftflow_protocol_core::datatypes::*;
#[allow(unused_imports)]
use craftflow_protocol_core::*;

#[derive(Debug, PartialEq, Clone)]
pub struct EncryptionBeginV00047 {
	pub shared_secret: Buffer<VarInt>,
	pub verify_token: Buffer<VarInt>,
}

impl MCPWrite for EncryptionBeginV00047 {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		let mut written_bytes = 0;

		written_bytes += self.shared_secret.write(output)?;
		written_bytes += self.verify_token.write(output)?;

		Ok(written_bytes)
	}
}

impl MCPRead for EncryptionBeginV00047 {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input, shared_secret) = Buffer::<VarInt>::read(input)?;
		let (input, verify_token) = Buffer::<VarInt>::read(input)?;

		Ok((
			input,
			Self {
				shared_secret,
				verify_token,
			},
		))
	}
}

impl crate::IntoVersionEnum for EncryptionBeginV00047 {
	type Packet = super::super::EncryptionBegin;

	fn into_version_enum(self) -> Self::Packet {
		super::super::EncryptionBegin::V00047(self)
	}
}
impl crate::IntoPacketEnum for EncryptionBeginV00047 {
	type State = super::super::super::Login;

	fn into_packet_enum(self) -> Self::State {
		let packet = crate::IntoVersionEnum::into_version_enum(self);
		super::super::super::Login::EncryptionBegin(packet)
	}
}
impl crate::IntoStateEnum for EncryptionBeginV00047 {
	type Direction = super::super::super::super::C2S;

	fn into_state_enum(self) -> Self::Direction {
		let state = crate::IntoPacketEnum::into_packet_enum(self);
		super::super::super::super::C2S::Login(state)
	}
}
