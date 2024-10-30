use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, State, WriteResult};
use anyhow::Result;
use craftflow_protocol_core::datatypes::Array;
use craftflow_protocol_versions::{
	s2c::{
		login::{
			success::{
				v00573::SuccessV00005, v00758::SuccessV00735, v00759, v00765::SuccessV00759,
				v00766, v00767::SuccessV00766,
			},
			Success,
		},
		Login,
	},
	IntoStateEnum, S2C,
};
use std::iter::{once, Once};

/// Indicates successful login and moves the state to Play/Configuration
#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbLoginSuccess {
	pub uuid: u128,
	pub username: String,
	pub properties: Vec<Property>,
	pub strict_error_handling: bool,
}

/// A property of the player
#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct Property {
	pub name: String,
	pub value: String,
	pub signature: Option<String>,
}

impl AbPacketWrite for AbLoginSuccess {
	type Direction = S2C;
	type Iter = Once<Self::Direction>;

	fn convert(self, protocol_version: u32, state: State) -> Result<WriteResult<Self::Iter>> {
		if state != State::Login {
			return Ok(WriteResult::Unsupported);
		}

		let pkt = match protocol_version {
			5..735 => SuccessV00005 {
				uuid: format!(
					"{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
					(self.uuid >> (4 * 24)) & 0xffff_ffff,
					(self.uuid >> (4 * 20)) & 0xffff,
					(self.uuid >> (4 * 16)) & 0xffff,
					(self.uuid >> (4 * 12)) & 0xffff,
					self.uuid & 0xffff_ffff_ffff
				),
				username: self.username,
			}
			.into_state_enum(),
			735..759 => SuccessV00735 {
				uuid: self.uuid,
				username: self.username,
			}
			.into_state_enum(),
			759..766 => SuccessV00759 {
				uuid: self.uuid,
				username: self.username,
				properties: Array::new(
					self.properties
						.into_iter()
						.map(|p| v00759::Property {
							name: p.name,
							value: p.value,
							signature: p.signature,
						})
						.collect(),
				),
			}
			.into_state_enum(),
			766.. => SuccessV00766 {
				uuid: self.uuid,
				username: self.username,
				properties: Array::new(
					self.properties
						.into_iter()
						.map(|p| v00766::Property {
							name: p.name,
							value: p.value,
							signature: p.signature,
						})
						.collect(),
				),
				strict_error_handling: self.strict_error_handling,
			}
			.into_state_enum(),
			_ => return Ok(WriteResult::Unsupported),
		};

		Ok(WriteResult::Success(once(pkt)))
	}
}

impl AbPacketNew for AbLoginSuccess {
	type Direction = S2C;
	type Constructor = NoConstructor<Self, S2C>;

	fn construct(
		packet: Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor, Self::Direction>> {
		Ok(match packet {
			S2C::Login(Login::Success(pkt)) => match pkt {
				Success::V00005(pkt) => ConstructorResult::Done(Self {
					uuid: u128::from_str_radix(&pkt.uuid.replace("-", ""), 16)?,
					username: pkt.username,
					properties: Vec::new(),
					strict_error_handling: true,
				}),
				Success::V00735(pkt) => ConstructorResult::Done(Self {
					uuid: pkt.uuid,
					username: pkt.username,
					properties: Vec::new(),
					strict_error_handling: true,
				}),
				Success::V00759(pkt) => ConstructorResult::Done(Self {
					uuid: pkt.uuid,
					username: pkt.username,
					properties: pkt
						.properties
						.data
						.into_iter()
						.map(|p| Property {
							name: p.name,
							value: p.value,
							signature: p.signature,
						})
						.collect(),
					strict_error_handling: true,
				}),
				Success::V00766(pkt) => ConstructorResult::Done(Self {
					uuid: pkt.uuid,
					username: pkt.username,
					properties: pkt
						.properties
						.data
						.into_iter()
						.map(|p| Property {
							name: p.name,
							value: p.value,
							signature: p.signature,
						})
						.collect(),
					strict_error_handling: pkt.strict_error_handling,
				}),
			},
			_ => ConstructorResult::Ignore(packet),
		})
	}
}
