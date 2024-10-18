pub mod settings;
include!(concat!(
	env!("OUT_DIR"),
	"/c2s/configuration/settings_enum.rs"
));
pub mod custom_payload;
include!(concat!(
	env!("OUT_DIR"),
	"/c2s/configuration/custom_payload_enum.rs"
));
pub mod finish_configuration;
include!(concat!(
	env!("OUT_DIR"),
	"/c2s/configuration/finish_configuration_enum.rs"
));
pub mod keep_alive;
include!(concat!(
	env!("OUT_DIR"),
	"/c2s/configuration/keep_alive_enum.rs"
));
pub mod pong;
include!(concat!(env!("OUT_DIR"), "/c2s/configuration/pong_enum.rs"));
