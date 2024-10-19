pub mod conf_disconnect;
pub mod conf_plugin;
pub mod login_compress;
pub mod login_disconnect;
pub mod login_encryption_begin;
pub mod login_plugin;
pub mod login_success;
pub mod status_info;
pub mod status_pong;

pub use conf_disconnect::AbConfDisconnect;
pub use conf_plugin::AbConfPlugin;
pub use login_compress::AbLoginCompress;
pub use login_disconnect::AbLoginDisconnect;
pub use login_encryption_begin::AbLoginEncryptionBegin;
pub use login_plugin::AbLoginPlugin;
pub use login_success::AbLoginSuccess;
pub use status_info::AbStatusInfo;
pub use status_pong::AbStatusPong;

include!("direction_macro.rs");

gen_direction_enum! {
	@DIRECTION=S2C;
	/// All packets that can be sent from the client to the server
	#[derive(Debug, Clone, PartialEq, Hash)]
	pub enum AbS2C {
		StatusInfo(AbStatusInfo),
		StatusPong(AbStatusPong),
		LoginDisconnect(AbLoginDisconnect),
		LoginEncryptionBegin(AbLoginEncryptionBegin),
		LoginSuccess(AbLoginSuccess),
		LoginCompress(AbLoginCompress),
		LoginPlugin(AbLoginPlugin),
		ConfPlugin(AbConfPlugin),
		ConfDisconnect(AbConfDisconnect),
	}
}
