use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, State, WriteResult};
use anyhow::{bail, Result};
use craftflow_protocol_core::datatypes::VarInt;
use craftflow_protocol_versions::{
	c2s::{
		configuration::{settings::v00764::SettingsV00764, Settings},
		Configuration,
	},
	IntoStateEnum, C2S,
};
use shallowclone::{MakeOwned, ShallowClone};
use std::{
	borrow::Cow,
	iter::{once, Once},
};

/// Client settings, can be send both during configuration or play states.
#[derive(ShallowClone, MakeOwned, Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbClientSettings<'a> {
	pub locale: Cow<'a, str>,
	pub view_distance: u8,
	pub chat_flags: ChatMode,
	pub chat_colors: bool,
	pub skin_parts: SkinParts,
	pub main_hand: MainHand,
	pub enable_text_filtering: bool,
	pub enable_server_listing: bool,
}

#[derive(ShallowClone, MakeOwned, Debug, Clone, Copy, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub enum ChatMode {
	Enabled = 0,
	CommandsOnly,
	Hidden,
}
#[derive(ShallowClone, MakeOwned, Debug, Clone, Copy, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub enum MainHand {
	Left = 0,
	Right,
}

#[derive(ShallowClone, MakeOwned, Debug, Clone, Copy, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct SkinParts {
	pub cape: bool,
	pub jacket: bool,
	pub left_sleeve: bool,
	pub right_sleeve: bool,
	pub left_pants: bool,
	pub right_pants: bool,
	pub hat: bool,
}

impl ChatMode {
	pub fn from_byte(byte: u8) -> Result<Self> {
		Ok(match byte {
			0 => Self::Enabled,
			1 => Self::CommandsOnly,
			2 => Self::Hidden,
			other => bail!("unknown chat mode {other:?}"),
		})
	}
}

impl MainHand {
	pub fn from_byte(byte: u8) -> Result<Self> {
		Ok(match byte {
			0 => Self::Left,
			1 => Self::Right,
			other => bail!("unknown main hand {other:?}"),
		})
	}
}

impl SkinParts {
	pub fn from_bitfield(bitfield: u8) -> Self {
		Self {
			cape: (bitfield & 1) != 0,
			jacket: (bitfield & 2) != 0,
			left_sleeve: (bitfield & 4) != 0,
			right_sleeve: (bitfield & 8) != 0,
			left_pants: (bitfield & 16) != 0,
			right_pants: (bitfield & 32) != 0,
			hat: (bitfield & 64) != 0,
		}
	}
	pub fn as_bitfield(self) -> u8 {
		(self.cape as u8) << 0
			| (self.jacket as u8) << 1
			| (self.left_sleeve as u8) << 2
			| (self.right_sleeve as u8) << 3
			| (self.left_pants as u8) << 4
			| (self.right_pants as u8) << 5
			| (self.hat as u8) << 6
	}
}

impl<'a> AbPacketWrite<'a> for AbClientSettings<'a> {
	type Direction = C2S<'a>;
	type Iter = Once<Self::Direction>;

	fn convert(&'a self, protocol_version: u32, state: State) -> Result<WriteResult<Self::Iter>> {
		let pkt = match state {
			State::Configuration => match protocol_version {
				764.. => SettingsV00764 {
					locale: self.locale.shallow_clone(),
					view_distance: self.view_distance as i8,
					chat_flags: VarInt(self.chat_flags as i32),
					chat_colors: self.chat_colors,
					skin_parts: self.skin_parts.as_bitfield(),
					main_hand: VarInt(self.main_hand as i32),
					enable_text_filtering: self.enable_text_filtering,
					enable_server_listing: self.enable_server_listing,
				}
				.into_state_enum(),
				_ => return Ok(WriteResult::Unsupported),
			},
			_ => return Ok(WriteResult::Unsupported),
		};

		Ok(WriteResult::Success(once(pkt)))
	}
}

impl<'a> AbPacketNew<'a> for AbClientSettings<'a> {
	type Direction = C2S<'a>;
	type Constructor = NoConstructor<Self, Self::Direction>;

	fn construct(
		packet: &'a Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor>> {
		Ok(match packet {
			C2S::Configuration(Configuration::Settings(pkt)) => match pkt {
				Settings::V00764(pkt) => ConstructorResult::Done(Self {
					locale: pkt.locale.shallow_clone(),
					view_distance: pkt.view_distance as u8,
					chat_flags: ChatMode::from_byte(pkt.chat_flags.0 as u8)?,
					chat_colors: pkt.chat_colors,
					skin_parts: SkinParts::from_bitfield(pkt.skin_parts),
					main_hand: MainHand::from_byte(pkt.main_hand.0 as u8)?,
					enable_text_filtering: pkt.enable_text_filtering,
					enable_server_listing: pkt.enable_server_listing,
				}),
			},
			_ => ConstructorResult::Ignore,
		})
	}
}
