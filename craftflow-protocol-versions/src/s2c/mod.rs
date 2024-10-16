pub mod status;
include!(concat!(env!("OUT_DIR"), "/s2c/status_enum.rs"));
pub mod login;
include!(concat!(env!("OUT_DIR"), "/s2c/login_enum.rs"));
pub mod configuration;
include!(concat!(env!("OUT_DIR"), "/s2c/configuration_enum.rs"));
