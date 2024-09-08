//! Implementation of Event for all packets
//! C2S packet events will be emitted after a packet is received from the client
//! S2C packet events will be emitted before a packet is sent to the client
//! `Post<S2C>` events will be emitted AFTER a packet is sent to the client

use crate::{reactor::Event, CraftFlow};
use craftflow_protocol::{
	protocol::{c2s, s2c, C2S, S2C},
	stable_packets::{self, c2s::legacy::LegacyPing, s2c::legacy::LegacyPingResponse},
	Packet,
};
use std::{marker::PhantomData, ops::ControlFlow};

impl<P: Packet + 'static> Event for P {
	/// The arguments for this event are the connection ID and the packet
	type Args = (usize, P);
	/// In the case of S2C packets, if the event is stopped, the packet will not be sent
	type Return = ();
}

/// `Post<Packet>` events are emitted after a packet is sent to the client
/// Contrary to the normal Packet events, which are emitted before the packet is sent
/// and can modify or stop the packet from being sent
pub struct Post<P: Packet<Direction = S2C>> {
	_phantom: PhantomData<P>,
}

impl<P: Packet<Direction = S2C> + 'static> Event for Post<P> {
	type Args = (usize, P);
	type Return = ();
}

pub(super) fn trigger_c2s(craftflow: &CraftFlow, conn_id: usize, packet: C2S) {
	macro_rules! e {
		($packet_type:path, $inner_packet:ident) => {{
			let _ = craftflow
				.reactor
				.event::<$packet_type>(&craftflow, (conn_id, $inner_packet));
		}};
	}

	match packet {
		C2S::LegacyPing(packet) => e!(LegacyPing, packet),
		C2S::Handshake(handshake) => e!(stable_packets::c2s::handshake::Handshake, handshake),
		C2S::Status(status) => match status {
			stable_packets::c2s::StatusPacket::StatusRequest { packet } => {
				e!(stable_packets::c2s::status::StatusRequest, packet)
			}
			stable_packets::c2s::StatusPacket::Ping { packet } => {
				e!(stable_packets::c2s::status::Ping, packet)
			}
		},
		C2S::Login(login) => match login {
			c2s::LoginPacket::LoginStart { packet } => e!(c2s::login::LoginStart, packet),
			c2s::LoginPacket::EncryptionResponse { packet } => {
				e!(c2s::login::EncryptionResponse, packet)
			}
			c2s::LoginPacket::PluginResponse { packet } => e!(c2s::login::PluginResponse, packet),
			c2s::LoginPacket::LoginAcknowledged { packet } => {
				e!(c2s::login::LoginAcknowledged, packet)
			}
			c2s::LoginPacket::CookieResponse { packet } => e!(c2s::login::CookieResponse, packet),
			c2s::LoginPacket::_Unsupported => unreachable!(),
		},
		// C2S::ConfigurationC2S(config) => match config {},
	}
}

// Oh my God...
macro_rules! trigger_s2c_gen {
	(@POST, $craftflow:ident, $conn_id:ident, $packet:ident) => {{
		macro_rules! e {
			($packet_type:path, $inner_packet:ident) => {
				match $craftflow
					.reactor
					.event::<Post<$packet_type>>(&$craftflow, ($conn_id, $inner_packet))
				{
					ControlFlow::Continue((_conn_id, packet)) => {
						ControlFlow::Continue(packet.into_packet_enum())
					}
					ControlFlow::Break(()) => ControlFlow::Break(()),
				}
			};
		}

		trigger_s2c_gen!(e, $craftflow, $conn_id, $packet)
	}};
	(@PRE, $craftflow:ident, $conn_id:ident, $packet:ident) => {{
		macro_rules! e {
			($packet_type:path, $inner_packet:ident) => {
				match $craftflow
					.reactor
					.event::<$packet_type>(&$craftflow, ($conn_id, $inner_packet))
				{
					ControlFlow::Continue((_conn_id, packet)) => {
						ControlFlow::Continue(packet.into_packet_enum())
					}
					ControlFlow::Break(()) => ControlFlow::Break(()),
				}
			};
		}

		trigger_s2c_gen!(e, $craftflow, $conn_id, $packet)
	}};
	($e:ident, $craftflow:ident, $conn_id:ident, $packet:ident) => {
		match $packet {
			S2C::LegacyPingResponse(packet) => $e!(LegacyPingResponse, packet),
			S2C::Status(status) => match status {
				stable_packets::s2c::StatusPacket::StatusResponse { packet } => {
					$e!(stable_packets::s2c::status::StatusResponse, packet)
				}
				stable_packets::s2c::StatusPacket::Pong { packet } => {
					$e!(stable_packets::s2c::status::Pong, packet)
				}
			},
			S2C::Login(login) => match login {
				s2c::LoginPacket::Disconnect { packet } => {
					$e!(s2c::login::Disconnect, packet)
				}
				s2c::LoginPacket::EncryptionRequest { packet } => {
					$e!(s2c::login::EncryptionRequest, packet)
				}
				s2c::LoginPacket::LoginSuccess { packet } => {
					$e!(s2c::login::LoginSuccess, packet)
				}
				s2c::LoginPacket::SetCompression { packet } => {
					$e!(s2c::login::SetCompression, packet)
				}
				s2c::LoginPacket::PluginRequest { packet } => {
					$e!(s2c::login::PluginRequest, packet)
				}
				s2c::LoginPacket::CookieRequest { packet } => {
					$e!(s2c::login::CookieRequest, packet)
				}
				s2c::LoginPacket::_Unsupported => unreachable!(),
			},
			// S2C::Configuration(config) => match config {},
		}
	};
}

pub(super) fn trigger_s2c_pre(
	craftflow: &CraftFlow,
	conn_id: usize,
	packet: S2C,
) -> ControlFlow<(), S2C> {
	trigger_s2c_gen!(@PRE, craftflow, conn_id, packet)
}
pub(super) fn trigger_s2c_post(
	craftflow: &CraftFlow,
	conn_id: usize,
	packet: S2C,
) -> ControlFlow<(), S2C> {
	trigger_s2c_gen!(@POST, craftflow, conn_id, packet)
}
