#[allow(unused_imports)]
use craftflow_protocol_core::datatypes::*;
#[allow(unused_imports)]
use craftflow_protocol_core::*;

#[derive(Debug, PartialEq, Clone)]
pub struct RemoveResourcePackV00765 {
	pub uuid: Option<u128>,
}

impl MCPWrite for RemoveResourcePackV00765 {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		self.uuid.write(output)
	}
}

impl MCPRead for RemoveResourcePackV00765 {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input, uuid) = Option::<u128>::read(input)?;
		Ok((input, Self { uuid }))
	}
}

impl crate::IntoVersionEnum for RemoveResourcePackV00765 {
	type Packet = super::super::RemoveResourcePack;

	fn into_version_enum(self) -> Self::Packet {
		super::super::RemoveResourcePack::V00765(self)
	}
}
impl crate::IntoPacketEnum for RemoveResourcePackV00765 {
	type State = super::super::super::Configuration;

	fn into_packet_enum(self) -> Self::State {
		let packet = crate::IntoVersionEnum::into_version_enum(self);
		super::super::super::Configuration::RemoveResourcePack(packet)
	}
}
impl crate::IntoStateEnum for RemoveResourcePackV00765 {
	type Direction = super::super::super::super::S2C;

	fn into_state_enum(self) -> Self::Direction {
		let state = crate::IntoPacketEnum::into_packet_enum(self);
		super::super::super::super::S2C::Configuration(state)
	}
}
