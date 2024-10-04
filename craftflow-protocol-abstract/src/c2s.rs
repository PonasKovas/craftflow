include!("direction_macro.rs");

pub mod handshake;
pub mod status_ping;
pub mod status_request_info;

pub use handshake::AbHandshake;
pub use status_ping::AbStatusPing;
pub use status_request_info::AbStatusRequestInfo;

gen_direction_enum! {
	@DIRECTION=C2S;
	/// All packets that can be sent from the client to the server
	#[derive(Debug, Clone, PartialEq, Hash)]
	pub enum AbC2S {
		Handshake(AbHandshake),
		StatusPing(AbStatusPing),
		StatusRequestInfo(AbStatusRequestInfo),
	}
}
