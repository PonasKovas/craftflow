pub mod custom_payload;
include!(concat!(
	env!("OUT_DIR"),
	"/s2c/configuration/custom_payload_enum.rs"
));
pub mod disconnect;
include!(concat!(
	env!("OUT_DIR"),
	"/s2c/configuration/disconnect_enum.rs"
));
pub mod finish_configuration;
include!(concat!(
	env!("OUT_DIR"),
	"/s2c/configuration/finish_configuration_enum.rs"
));
pub mod keep_alive;
include!(concat!(
	env!("OUT_DIR"),
	"/s2c/configuration/keep_alive_enum.rs"
));
pub mod ping;
include!(concat!(env!("OUT_DIR"), "/s2c/configuration/ping_enum.rs"));
pub mod registry_data;
include!(concat!(
	env!("OUT_DIR"),
	"/s2c/configuration/registry_data_enum.rs"
));
pub mod remove_resource_pack;
include!(concat!(
	env!("OUT_DIR"),
	"/s2c/configuration/remove_resource_pack_enum.rs"
));
pub mod add_resource_pack;
include!(concat!(
	env!("OUT_DIR"),
	"/s2c/configuration/add_resource_pack_enum.rs"
));
