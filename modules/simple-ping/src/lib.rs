#![doc(
	html_favicon_url = "https://github.com/PonasKovas/craftflow/blob/master/assets/icon.png?raw=true"
)]
#![doc(
	html_logo_url = "https://github.com/PonasKovas/craftflow/blob/master/assets/icon.png?raw=true"
)]

mod legacy_ping;
mod ping;
mod status;

use craftflow::CraftFlow;
use text::{Text, text};

craftflow::init!();

/// A simple ping module
/// Responds to the ping packet with a simple fixed message, shows the true online player count.
pub struct SimplePing {
	server_description: Text<'static>,
	favicon: Option<Vec<u8>>,
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
	pub fn set_favicon(mut self, favicon: Option<Vec<u8>>) -> Self {
		self.favicon = favicon;
		self
	}
	/// Adds the module to a CraftFlow instance.
	pub fn register(self, craftflow: &mut CraftFlow) {
		craftflow.modules.register(self);

		craftflow::reg!(to: &mut craftflow.reactor);
	}
}

impl Default for SimplePing {
	fn default() -> Self {
		Self::new()
	}
}
