//! Implementation of Event for all receivable packets

use crate::{
	reactor::{Event, Reactor},
	CFState,
};
use craftflow_protocol::packets::{
	handshake::{Handshake, HandshakeC2S},
	legacy::LegacyPing,
	login::{
		CookieResponse, EncryptionResponse, LoginAcknowledged, LoginC2S, LoginStart, PluginResponse,
	},
	status::{Ping, StatusC2S, StatusRequest},
	IntoPacketC2S, PacketC2S,
};

impl<P: IntoPacketC2S + 'static> Event for P {
	/// The arguments for this event are the connection ID and the packet
	type Args = (usize, P);
	type Return = ();
}

pub(super) fn trigger_packet_event(
	reactor: &mut Reactor<CFState>,
	state: &mut CFState,
	conn_id: usize,
	packet: PacketC2S,
) {
	macro_rules! e {
		($packet_type:path, $packet:ident) => {{
			let _ = reactor.event::<$packet_type>(state, &mut (conn_id, $packet));
		}};
	}

	match packet {
		PacketC2S::Legacy(packet) => e!(LegacyPing, packet),
		PacketC2S::HandshakeC2S(handshake) => match handshake {
			HandshakeC2S::Handshake(packet) => e!(Handshake, packet),
		},
		PacketC2S::StatusC2S(status) => match status {
			StatusC2S::StatusRequest(packet) => e!(StatusRequest, packet),
			StatusC2S::Ping(packet) => e!(Ping, packet),
		},
		PacketC2S::LoginC2S(login) => match login {
			LoginC2S::LoginStart(packet) => e!(LoginStart, packet),
			LoginC2S::EncryptionResponse(packet) => e!(EncryptionResponse, packet),
			LoginC2S::PluginResponse(packet) => e!(PluginResponse, packet),
			LoginC2S::LoginAcknowledged(packet) => e!(LoginAcknowledged, packet),
			LoginC2S::CookieResponse(packet) => e!(CookieResponse, packet),
		},
	}
}
