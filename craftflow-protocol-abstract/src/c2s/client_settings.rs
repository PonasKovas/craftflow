use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, WriteResult};
use anyhow::{bail, Result};
use craftflow_protocol_versions::{
	c2s::{
		configuration::{pong::v00767::PongV00764, Pong},
		Configuration,
	},
	IntoStateEnum, C2S,
};
use std::iter::{once, Once};

/// Client settings, can be send both during configuration or play states.
#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbClientSettings {
	pub locale: String,
	pub view_distance: u8,
	pub chat_flags: ChatMode,
	pub chat_colors: bool,
	pub skin_parts: SkinParts,
	pub main_hand: MainHand,
	pub enable_text_filtering: bool,
	pub enable_server_listing: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub enum ChatMode {
	Enabled = 0,
	CommandsOnly,
	Hidden,
}
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub enum MainHand {
	Left = 0,
	Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, PartialOrd, Ord)]
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

impl AbPacketWrite for AbClientSettings {
	type Direction = C2S;
	type Iter = Once<Self::Direction>;

	fn convert(self, protocol_version: u32) -> Result<WriteResult<Self::Iter>> {
		todo!()
		// let pkt = match protocol_version {
		// 	764.. => PongV00764 { id: self.id }.into_state_enum(),
		// 	_ => return Ok(WriteResult::Unsupported),
		// };

		// Ok(WriteResult::Success(once(pkt)))
	}
}

impl AbPacketNew for AbClientSettings {
	type Direction = C2S;
	type Constructor = NoConstructor<Self, C2S>;

	fn construct(
		packet: Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor, Self::Direction>> {
		todo!()
		// Ok(match packet {
		// 	C2S::Configuration(Configuration::Pong(pkt)) => match pkt {
		// 		Pong::V00764(pkt) => ConstructorResult::Done(Self { id: pkt.id }),
		// 	},
		// 	_ => ConstructorResult::Ignore(packet),
		// })
	}
}
