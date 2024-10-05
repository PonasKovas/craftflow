#[allow(unused_imports)]
use craftflow_protocol_core::datatypes::*;
#[allow(unused_imports)]
use craftflow_protocol_core::*;

#[derive(Debug, PartialEq, Clone, Hash, PartialOrd)]
pub struct EncryptionBeginV00047 {
	pub server_id: String,
	pub public_key: Buffer<VarInt>,
	pub verify_token: Buffer<VarInt>,
}
impl MCPWrite for EncryptionBeginV00047 {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		let mut written_bytes = 0;

		written_bytes += self.server_id.write(output)?;
		written_bytes += self.public_key.write(output)?;
		written_bytes += self.verify_token.write(output)?;

		Ok(written_bytes)
	}
}
impl MCPRead for EncryptionBeginV00047 {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input, server_id) = String::read(input)?;
		let (input, public_key) = Buffer::<VarInt>::read(input)?;
		let (input, verify_token) = Buffer::<VarInt>::read(input)?;

		Ok((
			input,
			Self {
				server_id,
				public_key,
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
	type Direction = super::super::super::super::S2C;

	fn into_state_enum(self) -> Self::Direction {
		let state = crate::IntoPacketEnum::into_packet_enum(self);
		super::super::super::super::S2C::Login(state)
	}
}
