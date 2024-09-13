use crate::{protocol::C2S, Packet};

/// This is a special packet with a different format that is sent by old clients to ping the server
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct LegacyPing;

impl Packet for LegacyPing {
	type Direction = C2S<'static>;
	type StaticSelf = LegacyPing;

	fn into_packet_enum(self) -> Self::Direction {
		C2S::LegacyPing(self)
	}
}
