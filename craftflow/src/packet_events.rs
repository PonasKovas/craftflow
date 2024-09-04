//! Implementation of Event for all packets
//! C2S packet events will be emitted after a packet is received from the client
//! S2C packet events will be emitted before a packet is sent to the client

use crate::{reactor::Event, CraftFlow};
use craftflow_protocol::packets::{
	handshake::{self, HandshakeC2S},
	legacy::{LegacyPing, LegacyPingResponse},
	login::{self, LoginC2S},
	status::{self, StatusC2S},
	IsPacket, PacketC2S, PacketS2C,
};
use craftflow_protocol::packets::{status::StatusS2C, IntoPacketS2C};
use std::ops::ControlFlow;

impl<P: IsPacket + 'static> Event for P {
	/// The arguments for this event are the connection ID and the packet
	type Args = (usize, P);
	/// In the case of S2C packets, if the event is stopped, the packet will not be sent
	type Return = ();
}

pub(super) fn trigger_c2s(craftflow: &CraftFlow, conn_id: usize, packet: PacketC2S) {
	macro_rules! e {
		($packet_type:path, $packet:ident) => {{
			let _ = craftflow
				.reactor
				.event::<$packet_type>(&craftflow.state, (conn_id, $packet));
		}};
	}

	match packet {
		PacketC2S::Legacy(packet) => e!(LegacyPing, packet),
		PacketC2S::HandshakeC2S(handshake) => match handshake {
			HandshakeC2S::Handshake(packet) => e!(handshake::Handshake, packet),
		},
		PacketC2S::StatusC2S(status) => match status {
			StatusC2S::StatusRequest(packet) => e!(status::StatusRequest, packet),
			StatusC2S::Ping(packet) => e!(status::Ping, packet),
		},
		PacketC2S::LoginC2S(login) => match login {
			LoginC2S::LoginStart(packet) => e!(login::LoginStart, packet),
			LoginC2S::EncryptionResponse(packet) => e!(login::EncryptionResponse, packet),
			LoginC2S::PluginResponse(packet) => e!(login::PluginResponse, packet),
			LoginC2S::LoginAcknowledged(packet) => e!(login::LoginAcknowledged, packet),
			LoginC2S::CookieResponse(packet) => e!(login::CookieResponse, packet),
		},
	}
}

pub(super) fn trigger_s2c(
	craftflow: &CraftFlow,
	conn_id: usize,
	packet: PacketS2C,
) -> ControlFlow<(), PacketS2C> {
	macro_rules! e {
		($packet_type:path, $packet:ident) => {{
			match craftflow
				.reactor
				.event::<$packet_type>(&craftflow.state, (conn_id, $packet))
			{
				ControlFlow::Continue((_conn_id, packet)) => {
					ControlFlow::Continue(packet.into_packet())
				}
				ControlFlow::Break(()) => ControlFlow::Break(()),
			}
		}};
	}

	match packet {
		PacketS2C::Legacy(packet) => e!(LegacyPingResponse, packet),
		PacketS2C::StatusS2C(status) => match status {
			StatusS2C::StatusResponse(packet) => e!(status::StatusResponse, packet),
			StatusS2C::Pong(packet) => e!(status::Pong, packet),
		},
		PacketS2C::LoginS2C(login) => match login {
			login::LoginS2C::Disconnect(packet) => e!(login::Disconnect, packet),
			login::LoginS2C::EncryptionRequest(packet) => e!(login::EncryptionRequest, packet),
			login::LoginS2C::LoginSuccess(packet) => e!(login::LoginSuccess, packet),
			login::LoginS2C::SetCompression(packet) => e!(login::SetCompression, packet),
			login::LoginS2C::PluginRequest(packet) => e!(login::PluginRequest, packet),
			login::LoginS2C::CookieRequest(packet) => e!(login::CookieRequest, packet),
		},
	}
}
