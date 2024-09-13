//! Implementation of `Event` for all packets
//! `C2S` packet events will be emitted after a packet is received from the client
//! `S2C` packet events will be emitted before a packet is sent to the client
//! `Post<S2C>` events will be emitted AFTER a packet is sent to the client

use crate::{reactor::Event, CraftFlow};
use craftflow_protocol::{
	protocol::{
		c2s::{self, handshake::Handshake, legacy::LegacyPing},
		s2c::{self, legacy::LegacyPingResponse},
		C2S, S2C,
	},
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
pub struct Post<P: Packet<Direction = S2C<'static>>> {
	_phantom: PhantomData<P>,
}

impl<P: Packet<Direction = S2C<'static>> + 'static> Event for Post<P> {
	type Args = (usize, P);
	type Return = ();
}

// FUCK RUST MACROS AND FUCK ENUMS AND FUCK DYN NOT SUPPORTING LIFETIMES
// FUCK YOU

pub(super) fn trigger_c2s<'a>(craftflow: &CraftFlow, conn_id: usize, packet: C2S<'a>) {
	fn inner<'a, P: Packet<Direction = C2S<'a>>>(
		packet: P,
		(craftflow, conn_id): (&CraftFlow, usize),
	) where
		<P as Packet>::StaticSelf: Packet<Direction = C2S<'a>> + 'static,
	{
		let _ = craftflow
			.reactor
			.event::<<P as Packet>::StaticSelf>(&craftflow, (conn_id, packet));
	}
	match packet {
		C2S::LegacyPing(legacy) => {
			inner::<LegacyPing>(legacy, (craftflow, conn_id));
		}
		C2S::Handshake(handshake) => {
			inner::<Handshake>(handshake, (craftflow, conn_id));
		}
		C2S::Status(status) => match status {
			c2s::StatusPacket::StatusRequest { packet } => {
				inner::<c2s::status::StatusRequest>(packet, (craftflow, conn_id));
			}
			c2s::StatusPacket::Ping { packet } => {
				inner::<c2s::status::Ping>(packet, (craftflow, conn_id));
			}
		},
		C2S::Login(login) => {
			craftflow_protocol::destructure_packet_c2s_login!(login, inner, (craftflow, conn_id),);
		}
		C2S::Configuration(configuration) => {
			craftflow_protocol::destructure_packet_c2s_configuration!(
				configuration,
				inner,
				(craftflow, conn_id),
			);
		}
	}
}

macro_rules! gen_trigger_s2c {
	($inner:ident, $packet:ident, $craftflow:ident, $conn_id:ident) => {{
		let r = match $packet {
			S2C::LegacyPingResponse(legacy) => {
				Some($inner::<LegacyPingResponse>(legacy, ($craftflow, $conn_id)))
			}
			S2C::Status(status) => Some(match status {
				s2c::StatusPacket::StatusResponse { packet } => {
					$inner::<s2c::status::StatusResponse>(packet, ($craftflow, $conn_id))
				}
				s2c::StatusPacket::Pong { packet } => {
					$inner::<s2c::status::Pong>(packet, ($craftflow, $conn_id))
				}
			}),
			S2C::Login(login) => {
				craftflow_protocol::destructure_packet_s2c_login!(
					login,
					$inner,
					($craftflow, $conn_id),
				)
			}
			S2C::Configuration(configuration) => {
				craftflow_protocol::destructure_packet_s2c_configuration!(
					configuration,
					$inner,
					($craftflow, $conn_id),
				)
			}
		};

		match r {
			Some(r) => r,
			None => {
				// this means we got an _Unsupported packet
				// we are the ones sending the packet so this means this is a retard incident
				panic!("DO NOT FUCKING SEND _Unsupported PACKETS!!!!");
			}
		}
	}};
}

pub(super) fn trigger_s2c_pre<'a>(
	craftflow: &CraftFlow,
	conn_id: usize,
	packet: S2C<'a>,
) -> ControlFlow<(), S2C<'a>> {
	fn inner<'a, P: Packet<Direction = S2C<'a>> + 'static>(
		packet: P,
		(craftflow, conn_id): (&CraftFlow, usize),
	) -> ControlFlow<(), S2C<'a>> {
		match craftflow.reactor.event::<P>(&craftflow, (conn_id, packet)) {
			ControlFlow::Continue((_conn_id, packet)) => {
				ControlFlow::Continue(packet.into_packet_enum())
			}
			ControlFlow::Break(()) => ControlFlow::Break(()),
		}
	}

	gen_trigger_s2c!(inner, packet, craftflow, conn_id)
}

pub(super) fn trigger_s2c_post<'a>(
	craftflow: &CraftFlow,
	conn_id: usize,
	packet: S2C<'a>,
) -> ControlFlow<(), S2C<'a>> {
	fn inner<'a, P: Packet<Direction = S2C<'a>> + 'static>(
		packet: P,
		(craftflow, conn_id): (&CraftFlow, usize),
	) -> ControlFlow<(), S2C<'a>> {
		match craftflow
			.reactor
			.event::<Post<P>>(&craftflow, (conn_id, packet))
		{
			ControlFlow::Continue((_conn_id, packet)) => {
				ControlFlow::Continue(packet.into_packet_enum())
			}
			ControlFlow::Break(()) => ControlFlow::Break(()),
		}
	}

	gen_trigger_s2c!(inner, packet, craftflow, conn_id)
}
