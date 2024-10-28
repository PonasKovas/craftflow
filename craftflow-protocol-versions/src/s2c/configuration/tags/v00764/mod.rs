#[allow(unused_imports)]
use crate::types::v00764::*;
#[allow(unused_imports)]
use craftflow_protocol_core::datatypes::*;
#[allow(unused_imports)]
use craftflow_protocol_core::*;

#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
pub struct TagsV00764 {
	pub registries: Array<VarInt, TagsRegistry>,
}

#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
pub struct TagsRegistry {
	pub name: String,
	pub tags: Tags,
}

impl MCPWrite for TagsV00764 {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		self.registries.write(output)
	}
}
impl MCPWrite for TagsRegistry {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		let mut written = 0;
		written += self.name.write(output)?;
		written += self.tags.write(output)?;

		Ok(written)
	}
}

impl MCPRead for TagsV00764 {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input, tags) = Array::<VarInt, _>::read(input)?;
		Ok((input, Self { registries: tags }))
	}
}
impl MCPRead for TagsRegistry {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input, registry_name) = String::read(input)?;
		let (input, tags) = Tags::read(input)?;
		Ok((
			input,
			Self {
				name: registry_name,
				tags,
			},
		))
	}
}

impl crate::IntoVersionEnum for TagsV00764 {
	type Packet = super::super::Tags;

	fn into_version_enum(self) -> Self::Packet {
		super::super::Tags::V00764(self)
	}
}
impl crate::IntoPacketEnum for TagsV00764 {
	type State = super::super::super::Configuration;

	fn into_packet_enum(self) -> Self::State {
		let packet = crate::IntoVersionEnum::into_version_enum(self);
		super::super::super::Configuration::Tags(packet)
	}
}
impl crate::IntoStateEnum for TagsV00764 {
	type Direction = super::super::super::super::S2C;

	fn into_state_enum(self) -> Self::Direction {
		let state = crate::IntoPacketEnum::into_packet_enum(self);
		super::super::super::super::S2C::Configuration(state)
	}
}
