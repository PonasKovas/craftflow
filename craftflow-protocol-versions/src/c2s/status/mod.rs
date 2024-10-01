pub mod ping_start;
include!(concat!(env!("OUT_DIR"), "/c2s/status/ping_start_enum.rs"));

pub mod ping;
include!(concat!(env!("OUT_DIR"), "/c2s/status/ping_enum.rs"));
