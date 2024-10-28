use craftflow_nbt::DynNBT;
#[allow(unused_imports)]
use craftflow_protocol_core::datatypes::*;
#[allow(unused_imports)]
use craftflow_protocol_core::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone)]
pub struct RegistryDataV00764 {
	pub inner: AnonymousNbt<RegistryStructure>,
}

// retarded ass structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegistryStructure {
	#[serde(rename = "minecraft:trim_material")]
	pub trim_material: InnerRegistryStructure,

	#[serde(rename = "minecraft:trim_pattern")]
	pub trim_pattern: InnerRegistryStructure,

	#[serde(rename = "minecraft:worldgen/biome")]
	pub biome: InnerRegistryStructure,

	#[serde(rename = "minecraft:chat_type")]
	pub chat_type: InnerRegistryStructure,

	#[serde(rename = "minecraft:damage_type")]
	pub damage_type: InnerRegistryStructure,

	#[serde(rename = "minecraft:dimension_type")]
	pub dimension_type: InnerRegistryStructure,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InnerRegistryStructure {
	#[serde(rename = "type")]
	pub registry_type: String,
	pub value: Vec<RegistryValueStructure>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegistryValueStructure {
	pub name: String,
	pub id: i32,
	pub element: DynNBT,
}

impl MCPWrite for RegistryDataV00764 {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		self.inner.write(output)
	}
}

impl MCPRead for RegistryDataV00764 {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input, inner) = AnonymousNbt::read(input)?;
		Ok((input, Self { inner }))
	}
}

impl crate::IntoVersionEnum for RegistryDataV00764 {
	type Packet = super::super::RegistryData;

	fn into_version_enum(self) -> Self::Packet {
		super::super::RegistryData::V00764(self)
	}
}
impl crate::IntoPacketEnum for RegistryDataV00764 {
	type State = super::super::super::Configuration;

	fn into_packet_enum(self) -> Self::State {
		let packet = crate::IntoVersionEnum::into_version_enum(self);
		super::super::super::Configuration::RegistryData(packet)
	}
}
impl crate::IntoStateEnum for RegistryDataV00764 {
	type Direction = super::super::super::super::S2C;

	fn into_state_enum(self) -> Self::Direction {
		let state = crate::IntoPacketEnum::into_packet_enum(self);
		super::super::super::super::S2C::Configuration(state)
	}
}
