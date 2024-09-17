mod legacy_ping;
mod ping;
mod status;

use base64::Engine;
use craftflow::CraftFlow;
use craftflow_protocol::{
	datatypes::Text,
	legacy::LegacyPing,
	protocol::c2s::status::{Ping, StatusRequest},
	text,
};
use legacy_ping::legacy_ping;

/// A simple ping module
/// Responds to the ping packet with a simple fixed message, shows the true online player count.
pub struct SimplePing {
	server_description: Text,
	favicon: Option<String>,
}

impl SimplePing {
	/// Creates a new default configuration for the simple ping.
	pub fn new() -> Self {
		Self {
			server_description: text!("<", obfuscated, font = "minecraft:alt", color = "white")
				+ text!(" A CraftFlow Server ", bold, color = "gold")
				+ text!(">", obfuscated, font = "minecraft:alt", color = "white"),
			favicon: Some(format!(
				"data:image/png;base64,{}",
				base64::prelude::BASE64_STANDARD.encode(include_bytes!("../../../assets/icon.png"))
			)),
		}
	}
	/// Sets the description for the server.
	pub fn set_description(mut self, description: Text) -> Self {
		self.server_description = description;
		self
	}
	/// Sets the favicon for the server.
	/// The favicon should be the raw PNG image (exactly 64x64 pixels).
	pub fn set_favicon(mut self, favicon: Option<&[u8]>) -> Self {
		self.favicon = favicon.map(|bytes| {
			format!(
				"data:image/png;base64,{}",
				base64::prelude::BASE64_STANDARD.encode(bytes)
			)
		});
		self
	}
	/// Adds the module to a CraftFlow instance.
	pub fn register(self, craftflow: &mut CraftFlow) {
		craftflow.modules.register(self);

		craftflow.reactor.add_handler::<LegacyPing, _>(legacy_ping);
		craftflow
			.reactor
			.add_handler::<StatusRequest, _>(status::status);
		craftflow.reactor.add_handler::<Ping, _>(ping::ping);
	}
}

impl Default for SimplePing {
	fn default() -> Self {
		Self::new()
	}
}
