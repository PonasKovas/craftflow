pub mod status_info;
pub mod status_pong;

pub use status_info::AbStatusInfo;
pub use status_pong::AbStatusPong;

include!("direction_macro.rs");

gen_direction_enum! {
	@DIRECTION=S2C;
	/// All packets that can be sent from the client to the server
	#[derive(Debug, Clone, PartialEq, Hash)]
	pub enum AbS2C {
		StatusInfo(AbStatusInfo),
		StatusPong(AbStatusPong),
	}
}
