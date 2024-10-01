pub mod server_info;
include!(concat!(env!("OUT_DIR"), "/s2c/status/server_info_enum.rs"));
pub mod ping;
include!(concat!(env!("OUT_DIR"), "/s2c/status/ping_enum.rs"));
