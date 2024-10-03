include!("direction_macro.rs");

pub mod handshake;

pub use handshake::AbHandshake;

gen_direction_enum! {
	@DIRECTION=C2S;
	/// All packets that can be sent from the client to the server
	#[derive(Debug, Clone, PartialEq, Hash)]
	pub enum AbC2S {
		Handshake(AbHandshake),
	}
}
