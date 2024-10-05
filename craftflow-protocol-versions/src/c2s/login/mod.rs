pub mod login_start;
include!(concat!(env!("OUT_DIR"), "/c2s/login/login_start_enum.rs"));
pub mod encryption_begin;
include!(concat!(
	env!("OUT_DIR"),
	"/c2s/login/encryption_begin_enum.rs"
));
