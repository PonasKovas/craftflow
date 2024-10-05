pub mod disconnect;
include!(concat!(env!("OUT_DIR"), "/s2c/login/disconnect_enum.rs"));
pub mod encryption_begin;
include!(concat!(
	env!("OUT_DIR"),
	"/s2c/login/encryption_begin_enum.rs"
));
pub mod success;
include!(concat!(env!("OUT_DIR"), "/s2c/login/success_enum.rs"));
pub mod compress;
include!(concat!(env!("OUT_DIR"), "/s2c/login/compress_enum.rs"));
