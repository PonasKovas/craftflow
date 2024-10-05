#[allow(unused_imports)]
use craftflow_protocol_core::datatypes::*;
#[allow(unused_imports)]
use craftflow_protocol_core::*;

#[derive(Debug, PartialEq, Clone, Hash, PartialOrd)]
pub struct CompressV00047 {
	pub threshold: VarInt,
}
impl MCPWrite for CompressV00047 {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		self.threshold.write(output)
	}
}
impl MCPRead for CompressV00047 {
	fn read(input: &[u8]) -> Result<(&[u8], Self)> {
		let (input, threshold) = VarInt::read(input)?;
		Ok((input, Self { threshold }))
	}
}

impl crate::IntoVersionEnum for CompressV00047 {
	type Packet = super::super::Compress;

	fn into_version_enum(self) -> Self::Packet {
		super::super::Compress::V00047(self)
	}
}
impl crate::IntoPacketEnum for CompressV00047 {
	type State = super::super::super::Login;

	fn into_packet_enum(self) -> Self::State {
		let packet = crate::IntoVersionEnum::into_version_enum(self);
		super::super::super::Login::Compress(packet)
	}
}
impl crate::IntoStateEnum for CompressV00047 {
	type Direction = super::super::super::super::S2C;

	fn into_state_enum(self) -> Self::Direction {
		let state = crate::IntoPacketEnum::into_packet_enum(self);
		super::super::super::super::S2C::Login(state)
	}
}
