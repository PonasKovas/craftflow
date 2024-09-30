include!("direction_macro.rs");

gen_direction_enum! {
	@DIRECTION=C2S;
	/// All packets that can be sent from the client to the server
	pub enum AbS2C {}
}
