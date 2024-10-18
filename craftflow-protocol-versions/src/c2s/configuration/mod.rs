pub mod settings;
include!(concat!(
	env!("OUT_DIR"),
	"/c2s/configuration/settings_enum.rs"
));
