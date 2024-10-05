pub mod handshaking;
include!(concat!(env!("OUT_DIR"), "/c2s/handshaking_enum.rs"));
pub mod status;
include!(concat!(env!("OUT_DIR"), "/c2s/status_enum.rs"));
pub mod login;
include!(concat!(env!("OUT_DIR"), "/c2s/login_enum.rs"));
