#[allow(unused_imports)]
use craftflow_protocol_core::datatypes::*;
#[allow(unused_imports)]
use craftflow_protocol_core::*;

#[derive(Debug, PartialEq, Clone)]
pub struct RegistryDataV00764 {
	pub codec: AnonymousNbt,
}

impl MCPWrite for RegistryDataV00764 {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		self.codec.write(output)
	}
}

impl MCPRead for RegistryDataV00764 {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input, codec) = AnonymousNbt::read(input)?;
		Ok((input, Self { codec }))
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
