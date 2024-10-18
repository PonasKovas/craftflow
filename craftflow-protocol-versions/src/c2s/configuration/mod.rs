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
