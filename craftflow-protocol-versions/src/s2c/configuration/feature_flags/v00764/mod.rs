#[allow(unused_imports)]
use craftflow_protocol_core::datatypes::*;
#[allow(unused_imports)]
use craftflow_protocol_core::*;

#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Eq, Ord)]
pub struct FeatureFlagsV00764 {
	pub features: Array<VarInt, String>,
}

impl MCPWrite for FeatureFlagsV00764 {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		self.features.write(output)
	}
}

impl MCPRead for FeatureFlagsV00764 {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input, features) = Array::<VarInt, String>::read(input)?;

		Ok((input, Self { features }))
	}
}

impl crate::IntoVersionEnum for FeatureFlagsV00764 {
	type Packet = super::super::FeatureFlags;

	fn into_version_enum(self) -> Self::Packet {
		super::super::FeatureFlags::V00764(self)
	}
}
impl crate::IntoPacketEnum for FeatureFlagsV00764 {
	type State = super::super::super::Configuration;

	fn into_packet_enum(self) -> Self::State {
		let packet = crate::IntoVersionEnum::into_version_enum(self);
		super::super::super::Configuration::FeatureFlags(packet)
	}
}
impl crate::IntoStateEnum for FeatureFlagsV00764 {
	type Direction = super::super::super::super::S2C;

	fn into_state_enum(self) -> Self::Direction {
		let state = crate::IntoPacketEnum::into_packet_enum(self);
		super::super::super::super::S2C::Configuration(state)
	}
}
