#[allow(unused_imports)]
use crate::types::v00764::*;
#[allow(unused_imports)]
use craftflow_protocol_core::datatypes::*;
#[allow(unused_imports)]
use craftflow_protocol_core::*;

#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
pub struct TagsV00764 {
	pub tags: Array<VarInt, TagContainer>,
}

#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
pub struct TagContainer {
	pub tag_type: String,
	pub tags: Tags,
}

impl MCPWrite for TagsV00764 {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		self.tags.write(output)
	}
}

impl MCPWrite for TagContainer {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		let mut written_bytes = 0;
		written_bytes += self.tag_type.write(output)?;
		written_bytes += self.tags.write(output)?;
		Ok(written_bytes)
	}
}

impl MCPRead for TagsV00764 {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input, tags) = Array::<VarInt, TagContainer>::read(input)?;
		Ok((input, Self { tags }))
	}
}

impl MCPRead for TagContainer {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input, tag_type) = String::read(input)?;
		let (input, tags) = Tags::read(input)?;
		Ok((input, Self { tag_type, tags }))
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
