use handshake::HandshakeC2S;
use legacy::{LegacyPing, LegacyPingResponse};
use login::{LoginC2S, LoginS2C};
use status::{StatusC2S, StatusS2C};

pub mod handshake;
pub mod legacy;
pub mod login;
pub mod status;

pub const PROTOCOL_VERSION: i32 = 767;

/// Convenience trait for converting a packet from any state into a general packet enum
pub trait IntoPacketC2S {
	fn into_packet(self) -> PacketC2S;
}
/// Convenience trait for converting a packet from any state into a general packet enum
pub trait IntoPacketS2C {
	fn into_packet(self) -> PacketS2C;
}

#[derive(Debug)]
pub enum PacketC2S {
	Legacy(LegacyPing),
	HandshakeC2S(HandshakeC2S),
	StatusC2S(StatusC2S),
	LoginC2S(LoginC2S),
}

#[derive(Debug)]
pub enum PacketS2C {
	Legacy(LegacyPingResponse),
	StatusS2C(StatusS2C),
	LoginS2C(LoginS2C),
}
