use craftflow_protocol_core::*;

#[derive(Debug, PartialEq)]
pub struct PingV00005 {
	pub time: i64,
}
impl MCPWrite for PingV00005 {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		self.time.write(output)
	}
}
impl MCPRead for PingV00005 {
	fn read(input: &[u8]) -> Result<(&[u8], Self)> {
		let (input, time) = i64::read(input)?;
		Ok((input, Self { time }))
	}
}

impl crate::IntoVersionEnum for PingV00005 {
	type Packet = super::super::Ping;

	fn into_version_enum(self) -> Self::Packet {
		super::super::Ping::V00005(self)
	}
}
impl crate::IntoPacketEnum for PingV00005 {
	type State = super::super::super::Status;

	fn into_packet_enum(self) -> Self::State {
		let packet = crate::IntoVersionEnum::into_version_enum(self);
		super::super::super::Status::Ping(packet)
	}
}
impl crate::IntoStateEnum for PingV00005 {
	type Direction = super::super::super::super::S2C;

	fn into_state_enum(self) -> Self::Direction {
		let state = crate::IntoPacketEnum::into_packet_enum(self);
		super::super::super::super::S2C::Status(state)
	}
}
