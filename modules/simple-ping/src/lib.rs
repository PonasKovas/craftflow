#![doc(
	html_favicon_url = "https://github.com/PonasKovas/craftflow/blob/master/assets/icon.png?raw=true"
)]
#![doc(
	html_logo_url = "https://github.com/PonasKovas/craftflow/blob/master/assets/icon.png?raw=true"
)]

mod legacy_ping;
mod ping;
mod status;

use craftflow::{
	connection::legacy::LegacyPing,
	packet_events::{C2SAbStatusPingEvent, C2SAbStatusRequestInfoEvent},
	CraftFlow,
};
use craftflow_protocol_core::{common_structures::Text, text};
use std::borrow::Cow;

/// A simple ping module
/// Responds to the ping packet with a simple fixed message, shows the true online player count.
pub struct SimplePing {
	server_description: Text<'static>,
	favicon: Option<Cow<'static, [u8]>>,
}

impl SimplePing {
	/// Creates a new default configuration for the simple ping.
	pub fn new() -> Self {
		Self {
			server_description: text!("<", obfuscated, font = "minecraft:alt", color = "white")
				+ text!(" A CraftFlow Server ", bold, color = "gold")
				+ text!(">", obfuscated, font = "minecraft:alt", color = "white"),
			favicon: Some(include_bytes!("../../../assets/icon64.png").into()),
		}
	}
	/// Sets the description for the server.
	pub fn set_description(mut self, description: Text<'static>) -> Self {
		self.server_description = description;
		self
	}
	/// Sets the favicon for the server.
	/// The favicon should be the raw PNG image (exactly 64x64 pixels).
	pub fn set_favicon(mut self, favicon: Option<Cow<'static, [u8]>>) -> Self {
		self.favicon = favicon;
		self
	}
	/// Adds the module to a CraftFlow instance.
	pub fn register(self, craftflow: &mut CraftFlow) {
		craftflow.modules.register(self);

		craftflow
			.reactor
			.add_handler::<LegacyPing, _>(legacy_ping::legacy_ping);
		craftflow
			.reactor
			.add_handler::<C2SAbStatusRequestInfoEvent, _>(status::status);
		craftflow
			.reactor
			.add_handler::<C2SAbStatusPingEvent, _>(ping::ping);
	}
}

impl Default for SimplePing {
	fn default() -> Self {
		Self::new()
	}
}
