pub mod conf_add_resource_pack;
pub mod conf_disconnect;
pub mod conf_feature_flags;
pub mod conf_finish;
pub mod conf_keepalive;
pub mod conf_ping;
pub mod conf_plugin;
pub mod conf_registry;
pub mod conf_tags;
pub mod login_compress;
pub mod login_disconnect;
pub mod login_encryption_begin;
pub mod login_plugin;
pub mod login_success;
pub mod status_info;
pub mod status_pong;

pub use conf_add_resource_pack::AbConfAddResourcePack;
pub use conf_disconnect::AbConfDisconnect;
pub use conf_feature_flags::AbConfFeatureFlags;
pub use conf_finish::AbConfFinish;
pub use conf_plugin::AbConfPlugin;
pub use conf_tags::AbConfTags;
pub use login_compress::AbLoginCompress;
pub use login_disconnect::AbLoginDisconnect;
pub use login_encryption_begin::AbLoginEncryptionBegin;
pub use login_plugin::AbLoginPluginRequest;
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
		LoginPluginRequest(AbLoginPluginRequest),
		ConfPlugin(AbConfPlugin),
		ConfDisconnect(AbConfDisconnect),
		ConfFinish(AbConfFinish),
		ConfAddResourcePack(AbConfAddResourcePack),
		ConfFeatureFlags(AbConfFeatureFlags),
		ConfTags(AbConfTags),
	}
}
