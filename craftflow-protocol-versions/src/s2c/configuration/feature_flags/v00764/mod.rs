
#[allow(unused_imports)]
use craftflow_protocol_core::datatypes::*;
#[allow(unused_imports)]
use craftflow_protocol_core::*;

#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Eq, Ord)]
pub struct FeatureFlagsV00764 {
	pub features: Vec<String>,
}

impl MCPWrite for FeatureFlagsV00764 {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		let mut written_bytes = 0;
		written_bytes += VarInt(self.features.len() as i32).write(output)?;
		for feature in &self.features {
			written_bytes += feature.write(output)?;
		}
		Ok(written_bytes)
	}
}

impl MCPRead for FeatureFlagsV00764 {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input, VarInt(count)) = VarInt::read(input)?;
		let mut features = Vec::with_capacity(count as usize);
		let mut current_input = input;
		for _ in 0..count {
			let (next_input, feature) = String::read(current_input)?;
			features.push(feature);
			current_input = next_input;
		}
		Ok((current_input, Self { features }))
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
