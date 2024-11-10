use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, State, WriteResult};
use anyhow::Result;
use craftflow_protocol_core::datatypes::Array;
use craftflow_protocol_versions::{
	s2c::{
		login::{
			success::{
				v00005::SuccessV00005,
				v00735::SuccessV00735,
				v00759::{self, SuccessV00759},
				v00766::{self, SuccessV00766},
			},
			Success,
		},
		Login,
	},
	IntoStateEnum, S2C,
};
use shallowclone::ShallowClone;
use std::{
	borrow::Cow,
	iter::{once, Once},
};

/// Indicates successful login and moves the state to Play/Configuration
#[derive(ShallowClone, Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbLoginSuccess<'a> {
	pub uuid: u128,
	pub username: Cow<'a, str>,
	// this could be made into a specialized cow, but i dont think its worth it
	pub properties: Vec<Property<'a>>,
	pub strict_error_handling: bool,
}

/// A property of the player
#[derive(ShallowClone, Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct Property<'a> {
	pub name: Cow<'a, str>,
	pub value: Cow<'a, str>,
	pub signature: Option<Cow<'a, str>>,
}

impl<'a> AbPacketWrite<'a> for AbLoginSuccess<'a> {
	type Direction = S2C<'a>;
	type Iter = Once<Self::Direction>;

	fn convert(&'a self, protocol_version: u32, state: State) -> Result<WriteResult<Self::Iter>> {
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
				)
				.into(),
				username: self.username.shallow_clone(),
			}
			.into_state_enum(),
			735..759 => SuccessV00735 {
				uuid: self.uuid,
				username: self.username.shallow_clone(),
			}
			.into_state_enum(),
			759..766 => SuccessV00759 {
				uuid: self.uuid,
				username: self.username.shallow_clone(),
				properties: Array::from(
					self.properties
						.iter()
						.map(|p| v00759::Property {
							name: p.name.shallow_clone(),
							value: p.value.shallow_clone(),
							signature: p.signature.shallow_clone(),
						})
						.collect::<Vec<_>>(),
				),
			}
			.into_state_enum(),
			766.. => SuccessV00766 {
				uuid: self.uuid,
				username: self.username.shallow_clone(),
				properties: Array::from(
					self.properties
						.iter()
						.map(|p| v00766::Property {
							name: p.name.shallow_clone(),
							value: p.value.shallow_clone(),
							signature: p.signature.shallow_clone(),
						})
						.collect::<Vec<_>>(),
				),
				strict_error_handling: self.strict_error_handling,
			}
			.into_state_enum(),
			_ => return Ok(WriteResult::Unsupported),
		};

		Ok(WriteResult::Success(once(pkt)))
	}
}

impl<'a> AbPacketNew<'a> for AbLoginSuccess<'a> {
	type Direction = S2C<'a>;
	type Constructor = NoConstructor<Self, S2C<'a>>;

	fn construct(
		packet: &'a Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor>> {
		Ok(match packet {
			S2C::Login(Login::Success(pkt)) => match pkt {
				Success::V00005(pkt) => ConstructorResult::Done(Self {
					uuid: u128::from_str_radix(&pkt.uuid.replace("-", ""), 16)?,
					username: pkt.username.shallow_clone(),
					properties: Vec::new(),
					strict_error_handling: true,
				}),
				Success::V00735(pkt) => ConstructorResult::Done(Self {
					uuid: pkt.uuid,
					username: pkt.username.shallow_clone(),
					properties: Vec::new(),
					strict_error_handling: true,
				}),
				Success::V00759(pkt) => ConstructorResult::Done(Self {
					uuid: pkt.uuid,
					username: pkt.username.shallow_clone(),
					properties: pkt
						.properties
						.iter()
						.map(|p| Property {
							name: p.name.shallow_clone(),
							value: p.value.shallow_clone(),
							signature: p.signature.shallow_clone(),
						})
						.collect(),
					strict_error_handling: true,
				}),
				Success::V00766(pkt) => ConstructorResult::Done(Self {
					uuid: pkt.uuid,
					username: pkt.username.shallow_clone(),
					properties: pkt
						.properties
						.iter()
						.map(|p| Property {
							name: p.name.shallow_clone(),
							value: p.value.shallow_clone(),
							signature: p.signature.shallow_clone(),
						})
						.collect(),
					strict_error_handling: pkt.strict_error_handling,
				}),
			},
			_ => ConstructorResult::Ignore,
		})
	}
}
