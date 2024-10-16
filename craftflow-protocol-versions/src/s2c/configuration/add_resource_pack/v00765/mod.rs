#[allow(unused_imports)]
use craftflow_protocol_core::datatypes::*;
#[allow(unused_imports)]
use craftflow_protocol_core::*;

#[derive(Debug, PartialEq, Clone)]
pub struct AddResourcePackV00765 {
	pub uuid: u128,
	pub url: String,
	pub hash: String,
	pub forced: bool,
	pub prompt_message: Option<AnonymousNbt>,
}

impl MCPWrite for AddResourcePackV00765 {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		let mut written_bytes = 0;

		written_bytes += self.uuid.write(output)?;
		written_bytes += self.url.write(output)?;
		written_bytes += self.hash.write(output)?;
		written_bytes += self.forced.write(output)?;
		written_bytes += self.prompt_message.write(output)?;

		Ok(written_bytes)
	}
}

impl MCPRead for AddResourcePackV00765 {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input, uuid) = u128::read(input)?;
		let (input, url) = String::read(input)?;
		let (input, hash) = String::read(input)?;
		let (input, forced) = bool::read(input)?;
		let (input, prompt_message) = Option::<AnonymousNbt>::read(input)?;

		Ok((
			input,
			Self {
				uuid,
				url,
				hash,
				forced,
				prompt_message,
			},
		))
	}
}

impl crate::IntoVersionEnum for AddResourcePackV00765 {
	type Packet = super::super::AddResourcePack;

	fn into_version_enum(self) -> Self::Packet {
		super::super::AddResourcePack::V00765(self)
	}
}
impl crate::IntoPacketEnum for AddResourcePackV00765 {
	type State = super::super::super::Configuration;

	fn into_packet_enum(self) -> Self::State {
		let packet = crate::IntoVersionEnum::into_version_enum(self);
		super::super::super::Configuration::AddResourcePack(packet)
	}
}
impl crate::IntoStateEnum for AddResourcePackV00765 {
	type Direction = super::super::super::super::S2C;

	fn into_state_enum(self) -> Self::Direction {
		let state = crate::IntoPacketEnum::into_packet_enum(self);
		super::super::super::super::S2C::Configuration(state)
	}
}
