pub mod set_protocol;
include!(concat!(
	env!("OUT_DIR"),
	"/c2s/handshaking/set_protocol_enum.rs"
));
