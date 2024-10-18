#[allow(unused_imports)]
use crate::types::v00765::*;
#[allow(unused_imports)]
use craftflow_protocol_core::datatypes::*;
#[allow(unused_imports)]
use craftflow_protocol_core::*;

#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
pub struct ResourcePackReceiveV00765 {
	pub uuid: u128,
	pub result: ResultType,
}

#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
pub enum ResultType {
	Success,
	Failure,
	Unknown(i32),
}

impl MCPWrite for ResourcePackReceiveV00765 {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		let mut written_bytes = 0;

		written_bytes += self.uuid.write(output)?;
		written_bytes += self.result.write(output)?;

		Ok(written_bytes)
	}
}

impl MCPWrite for ResultType {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		match self {
			ResultType::Success => VarInt(0).write(output),
			ResultType::Failure => VarInt(1).write(output),
			ResultType::Unknown(code) => VarInt(*code).write(output),
		}
	}
}

impl MCPRead for ResourcePackReceiveV00765 {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input, uuid) = u128::read(input)?;
		let (input, result) = ResultType::read(input)?;

		Ok((input, Self { uuid, result }))
	}
}

impl MCPRead for ResultType {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input, code) = VarInt::read(input)?;

		let result = match code.0 {
			0 => ResultType::Success,
			1 => ResultType::Failure,
			_ => ResultType::Unknown(code.0),
		};

		Ok((input, result))
	}
}

impl crate::IntoVersionEnum for ResourcePackReceiveV00765 {
	type Packet = super::super::ResourcePackReceive;

	fn into_version_enum(self) -> Self::Packet {
		super::super::ResourcePackReceive::V00765(self)
	}
}
impl crate::IntoPacketEnum for ResourcePackReceiveV00765 {
	type State = super::super::super::Configuration;

	fn into_packet_enum(self) -> Self::State {
		let packet = crate::IntoVersionEnum::into_version_enum(self);
		super::super::super::Configuration::ResourcePackReceive(packet)
	}
}
impl crate::IntoStateEnum for ResourcePackReceiveV00765 {
	type Direction = super::super::super::super::C2S;

	fn into_state_enum(self) -> Self::Direction {
		let state = crate::IntoPacketEnum::into_packet_enum(self);
		super::super::super::super::C2S::Configuration(state)
	}
}
