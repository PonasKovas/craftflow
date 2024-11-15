// pub mod client_settings;
// pub mod conf_finish;
// pub mod conf_keepalive;
// pub mod conf_plugin;
// pub mod conf_pong;
// pub mod conf_resource_pack_response;
pub mod handshake;
// pub mod login_acknowledge;
// pub mod login_encryption;
// pub mod login_plugin;
// pub mod login_start;
// pub mod status_ping;
// pub mod status_request_info;

// pub use client_settings::AbClientSettings;
// pub use conf_finish::AbConfFinish;
// pub use conf_keepalive::AbConfKeepAlive;
// pub use conf_plugin::AbConfPlugin;
// pub use conf_pong::AbConfPong;
// pub use conf_resource_pack_response::AbConfResourcePackResponse;
pub use handshake::AbHandshake;
// pub use login_acknowledge::AbLoginAcknowledge;
// pub use login_encryption::AbLoginEncryption;
// pub use login_plugin::AbLoginPluginResponse;
// pub use login_start::AbLoginStart;
// pub use status_ping::AbStatusPing;
// pub use status_request_info::AbStatusRequestInfo;

include!("direction_macro.rs");

use shallowclone::{MakeOwned, ShallowClone};
gen_direction_enum! {
	@DIRECTION=C2S;
	/// All packets that can be sent from the client to the server
	#[derive(ShallowClone, MakeOwned, Debug, Clone, PartialEq, Hash)]
	pub enum AbC2S<'a> {
		Handshake(AbHandshake<'a>),
		// StatusPing(AbStatusPing),
		// StatusRequestInfo(AbStatusRequestInfo),
		// LoginStart(AbLoginStart<'a>),
		// LoginEncryption(AbLoginEncryption<'a>),
		// LoginPluginResponse(AbLoginPluginResponse<'a>),
		// LoginAcknowledge(AbLoginAcknowledge),
		// ConfPlugin(AbConfPlugin<'a>),
		// ConfFinish(AbConfFinish),
		// ConfKeepAlive(AbConfKeepAlive),
		// ConfPong(AbConfPong),
		// ConfResourcePackResponse(AbConfResourcePackResponse),
		// ClientSettings(AbClientSettings<'a>),
	}
}
