
#[allow(unused_imports)]
use craftflow_protocol_core::datatypes::*;
#[allow(unused_imports)]
use craftflow_protocol_core::*;

#[derive(Debug, PartialEq, Clone)]
pub struct LoginStartV00759 {
	pub username: String,
	pub signature: Option<Signature>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Signature {
	pub timestamp: i64,
	pub public_key: Buffer<VarInt>,
	pub signature: Buffer<VarInt>,
}

impl MCPWrite for LoginStartV00759 {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		let mut written_bytes = 0;

		written_bytes += self.username.write(output)?;
		written_bytes += self.signature.write(output)?;

		Ok(written_bytes)
	}
}

impl MCPWrite for Signature {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		let mut written_bytes = 0;

		written_bytes += self.timestamp.write(output)?;
		written_bytes += self.public_key.write(output)?;
		written_bytes += self.signature.write(output)?;

		Ok(written_bytes)
	}
}

impl MCPRead for LoginStartV00759 {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input, username) = String::read(input)?;
		let (input, signature) = Option::<Signature>::read(input)?;

		Ok((
			input,
			Self {
				username,
				signature,
			},
		))
	}
}

impl MCPRead for Signature {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input, timestamp) = i64::read(input)?;
		let (input, public_key) = Buffer::<VarInt>::read(input)?;
		let (input, signature) = Buffer::<VarInt>::read(input)?;

		Ok((
			input,
			Self {
				timestamp,
				public_key,
				signature,
			},
		))
	}
}

impl crate::IntoVersionEnum for LoginStartV00759 {
	type Packet = super::super::LoginStart;

	fn into_version_enum(self) -> Self::Packet {
		super::super::LoginStart::V00759(self)
	}
}
impl crate::IntoPacketEnum for LoginStartV00759 {
	type State = super::super::super::Login;

	fn into_packet_enum(self) -> Self::State {
		let packet = crate::IntoVersionEnum::into_version_enum(self);
		super::super::super::Login::LoginStart(packet)
	}
}
impl crate::IntoStateEnum for LoginStartV00759 {
	type Direction = super::super::super::super::C2S;

	fn into_state_enum(self) -> Self::Direction {
		let state = crate::IntoPacketEnum::into_packet_enum(self);
		super::super::super::super::C2S::Login(state)
	}
}
