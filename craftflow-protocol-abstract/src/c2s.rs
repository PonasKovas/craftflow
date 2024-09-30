include!("direction_macro.rs");

mod handshake;

pub use handshake::AbHandshake;

gen_direction_enum! {
	@DIRECTION=C2S;
	/// All packets that can be sent from the client to the server
	pub enum AbC2S {
		Handshake(AbHandshake),
	}
}
