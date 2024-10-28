#[allow(unused_imports)]
use crate::types::v00766::*;
#[allow(unused_imports)]
use craftflow_protocol_core::common_structures::*;
#[allow(unused_imports)]
use craftflow_protocol_core::datatypes::*;
#[allow(unused_imports)]
use craftflow_protocol_core::*;

#[derive(Debug, PartialEq, Clone)]
pub struct RegistryDataV00766 {
	pub id: String,
	pub entries: Array<VarInt, Entry>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Entry {
	pub key: String,
	pub value: Option<AnonymousNbt>,
}

impl MCPWrite for RegistryDataV00766 {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		let mut written_bytes = 0;

		written_bytes += self.id.write(output)?;
		written_bytes += self.entries.write(output)?;

		Ok(written_bytes)
	}
}

impl MCPWrite for Entry {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		let mut written_bytes = 0;

		written_bytes += self.key.write(output)?;
		written_bytes += self.value.write(output)?;

		Ok(written_bytes)
	}
}

impl MCPRead for RegistryDataV00766 {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input, id) = String::read(input)?;
		let (input, entries) = Array::<VarInt, Entry>::read(input)?;

		Ok((input, Self { id, entries }))
	}
}

impl MCPRead for Entry {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input, key) = String::read(input)?;
		let (input, value) = Option::<AnonymousNbt>::read(input)?;

		Ok((input, Self { key, value }))
	}
}

impl crate::IntoVersionEnum for RegistryDataV00766 {
	type Packet = super::super::RegistryData;

	fn into_version_enum(self) -> Self::Packet {
		super::super::RegistryData::V00766(self)
	}
}
impl crate::IntoPacketEnum for RegistryDataV00766 {
	type State = super::super::super::Configuration;

	fn into_packet_enum(self) -> Self::State {
		let packet = crate::IntoVersionEnum::into_version_enum(self);
		super::super::super::Configuration::RegistryData(packet)
	}
}
impl crate::IntoStateEnum for RegistryDataV00766 {
	type Direction = super::super::super::super::S2C;

	fn into_state_enum(self) -> Self::Direction {
		let state = crate::IntoPacketEnum::into_packet_enum(self);
		super::super::super::super::S2C::Configuration(state)
	}
}
