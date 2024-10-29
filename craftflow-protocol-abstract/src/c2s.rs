pub mod conf_finish;
pub mod conf_plugin;
pub mod handshake;
pub mod login_acknowledge;
pub mod login_encryption;
pub mod login_plugin;
pub mod login_start;
pub mod status_ping;
pub mod status_request_info;

pub use conf_finish::AbConfFinish;
pub use conf_plugin::AbConfPlugin;
pub use handshake::AbHandshake;
pub use login_acknowledge::AbLoginAcknowledge;
pub use login_encryption::AbLoginEncryption;
pub use login_plugin::AbLoginPluginResponse;
pub use login_start::AbLoginStart;
pub use status_ping::AbStatusPing;
pub use status_request_info::AbStatusRequestInfo;

include!("direction_macro.rs");

gen_direction_enum! {
	@DIRECTION=C2S;
	/// All packets that can be sent from the client to the server
	#[derive(Debug, Clone, PartialEq, Hash)]
	pub enum AbC2S {
		Handshake(AbHandshake),
		StatusPing(AbStatusPing),
		StatusRequestInfo(AbStatusRequestInfo),
		LoginStart(AbLoginStart),
		LoginEncryption(AbLoginEncryption),
		LoginPluginResponse(AbLoginPluginResponse),
		LoginAcknowledge(AbLoginAcknowledge),
		ConfPlugin(AbConfPlugin),
		ConfFinish(AbConfFinish),
	}
}
