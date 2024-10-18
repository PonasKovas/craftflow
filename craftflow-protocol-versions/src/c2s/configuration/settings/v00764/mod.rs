#[allow(unused_imports)]
use crate::types::v00764::*;
#[allow(unused_imports)]
use craftflow_protocol_core::datatypes::*;
#[allow(unused_imports)]
use craftflow_protocol_core::*;

#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
pub struct SettingsV00764 {
	pub locale: String,
	pub view_distance: i8,
	pub chat_flags: VarInt,
	pub chat_colors: bool,
	pub skin_parts: u8,
	pub main_hand: VarInt,
	pub enable_text_filtering: bool,
	pub enable_server_listing: bool,
}

impl MCPWrite for SettingsV00764 {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		let mut written_bytes = 0;

		written_bytes += self.locale.write(output)?;
		written_bytes += self.view_distance.write(output)?;
		written_bytes += self.chat_flags.write(output)?;
		written_bytes += self.chat_colors.write(output)?;
		written_bytes += self.skin_parts.write(output)?;
		written_bytes += self.main_hand.write(output)?;
		written_bytes += self.enable_text_filtering.write(output)?;
		written_bytes += self.enable_server_listing.write(output)?;

		Ok(written_bytes)
	}
}

impl MCPRead for SettingsV00764 {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input, locale) = String::read(input)?;
		let (input, view_distance) = i8::read(input)?;
		let (input, chat_flags) = VarInt::read(input)?;
		let (input, chat_colors) = bool::read(input)?;
		let (input, skin_parts) = u8::read(input)?;
		let (input, main_hand) = VarInt::read(input)?;
		let (input, enable_text_filtering) = bool::read(input)?;
		let (input, enable_server_listing) = bool::read(input)?;

		Ok((
			input,
			Self {
				locale,
				view_distance,
				chat_flags,
				chat_colors,
				skin_parts,
				main_hand,
				enable_text_filtering,
				enable_server_listing,
			},
		))
	}
}

impl crate::IntoVersionEnum for SettingsV00764 {
	type Packet = super::super::Settings;

	fn into_version_enum(self) -> Self::Packet {
		super::super::Settings::V00764(self)
	}
}
impl crate::IntoPacketEnum for SettingsV00764 {
	type State = super::super::super::Configuration;

	fn into_packet_enum(self) -> Self::State {
		let packet = crate::IntoVersionEnum::into_version_enum(self);
		super::super::super::Configuration::Settings(packet)
	}
}
impl crate::IntoStateEnum for SettingsV00764 {
	type Direction = super::super::super::super::C2S;

	fn into_state_enum(self) -> Self::Direction {
		let state = crate::IntoPacketEnum::into_packet_enum(self);
		super::super::super::super::C2S::Configuration(state)
	}
}
