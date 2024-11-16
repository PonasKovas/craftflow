pub mod conf_add_resource_pack;
pub mod conf_feature_flags;
pub mod conf_finish;
pub mod conf_keepalive;
pub mod conf_ping;
pub mod conf_plugin;
pub mod conf_registry;
pub mod conf_remove_resource_pack;
pub mod conf_reset_chat;
pub mod conf_tags;
pub mod disconnect;
pub mod login_compress;
pub mod login_encryption_begin;
pub mod login_plugin;
pub mod login_success;
pub mod status_info;
pub mod status_pong;

pub use conf_add_resource_pack::AbConfAddResourcePack;
pub use conf_feature_flags::AbConfFeatureFlags;
pub use conf_finish::AbConfFinish;
use conf_keepalive::AbConfKeepAlive;
use conf_ping::AbConfPing;
pub use conf_plugin::AbConfPlugin;
use conf_registry::AbConfRegistry;
pub use conf_remove_resource_pack::AbConfRemoveResourcePack;
pub use conf_reset_chat::AbConfResetChat;
pub use conf_tags::AbConfTags;
pub use disconnect::AbDisconnect;
pub use login_compress::AbLoginCompress;
pub use login_encryption_begin::AbLoginEncryptionBegin;
pub use login_plugin::AbLoginPluginRequest;
pub use login_success::AbLoginSuccess;
pub use status_info::AbStatusInfo;
pub use status_pong::AbStatusPong;

include!("direction_macro.rs");

use shallowclone::{MakeOwned, ShallowClone};
gen_direction_enum! {
	@DIRECTION=S2C;
	/// All packets that can be sent from the client to the server
	#[derive(ShallowClone, MakeOwned, Debug, Clone, PartialEq)]
	pub enum AbS2C<'a> {
		Disconnect(AbDisconnect<'a>),

		StatusInfo(AbStatusInfo<'a>),
		StatusPong(AbStatusPong),

		LoginEncryptionBegin(AbLoginEncryptionBegin<'a>),
		LoginSuccess(AbLoginSuccess<'a>),
		LoginCompress(AbLoginCompress),
		LoginPluginRequest(AbLoginPluginRequest<'a>),

		ConfPlugin(AbConfPlugin<'a>),
		ConfFinish(AbConfFinish),
		ConfAddResourcePack(AbConfAddResourcePack<'a>),
		ConfRemoveResourcePack(AbConfRemoveResourcePack),
		ConfFeatureFlags(AbConfFeatureFlags),
		ConfTags(AbConfTags<'a>),
		ConfResetChat(AbConfResetChat),
		ConfRegistry(AbConfRegistry<'a>),
		ConfPing(AbConfPing),
		ConfKeepAlive(AbConfKeepAlive),
	}
}
