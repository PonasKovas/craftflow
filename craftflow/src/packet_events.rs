//! Implementation of `Event` for all packets
//! `C2S` packet events will be emitted after a packet is received from the client
//! `S2C` packet events will be emitted before a packet is sent to the client
//! `Post<S2C>` events will be emitted AFTER a packet is sent to the client

use crate::{reactor::Event, CraftFlow};
use craftflow_protocol_versions::{IntoStateEnum, S2C};
use std::{any::Any, marker::PhantomData, ops::ControlFlow};

impl<P: IntoStateEnum<Direction = S2C> + Any> Event for P {
	/// The arguments for this event are the connection ID and the packet
	type Args<'a> = (u64, &'a mut P);
	/// In the case of S2C packets, if the event is stopped, the packet will not be sent
	type Return = ();
}

/// `Post<Packet>` events are emitted after a packet is sent to the client
/// Contrary to the normal Packet events, which are emitted before the packet is sent
/// and can modify or stop the packet from being sent
pub struct Post<P: IntoStateEnum<Direction = S2C>> {
	_phantom: PhantomData<P>,
}

impl<P: IntoStateEnum<Direction = S2C> + Any> Event for Post<P> {
	type Args<'a> = (u64, &'a mut P);
	type Return = ();
}

// pub(super) fn trigger_c2s(craftflow: &CraftFlow, conn_id: u64, packet: C2S) {
// 	fn inner<'a, P: Packet<Direction = C2S> + 'static>(
// 		packet: P,
// 		(craftflow, conn_id): (&CraftFlow, u64),
// 	) {
// 		let _ = craftflow.reactor.event::<P>(&craftflow, (conn_id, packet));
// 	}
// 	match packet {
// 		C2S::LegacyPing(legacy) => {
// 			inner::<LegacyPing>(legacy, (craftflow, conn_id));
// 		}
// 		C2S::Handshake(handshake) => {
// 			craftflow_protocol::destructure_packet_c2s_handshake!(
// 				handshake,
// 				inner,
// 				(craftflow, conn_id),
// 			);
// 		}
// 		C2S::Status(status) => {
// 			craftflow_protocol::destructure_packet_c2s_status!(status, inner, (craftflow, conn_id),);
// 		}
// 		C2S::Login(login) => {
// 			craftflow_protocol::destructure_packet_c2s_login!(login, inner, (craftflow, conn_id),);
// 		}
// 		C2S::Configuration(configuration) => {
// 			craftflow_protocol::destructure_packet_c2s_configuration!(
// 				configuration,
// 				inner,
// 				(craftflow, conn_id),
// 			);
// 		}
// 		C2S::Play(play) => {
// 			craftflow_protocol::destructure_packet_c2s_play!(play, inner, (craftflow, conn_id),);
// 		}
// 	}
// }

// macro_rules! gen_trigger_s2c {
// 	($inner:ident, $packet:ident, $craftflow:ident, $conn_id:ident) => {{
// 		let r = match $packet {
// 			S2C::LegacyPingResponse(legacy) => {
// 				Some($inner::<LegacyPingResponse>(legacy, ($craftflow, $conn_id)))
// 			}
// 			S2C::Status(status) => {
// 				craftflow_protocol::destructure_packet_s2c_status!(
// 					status,
// 					$inner,
// 					($craftflow, $conn_id),
// 				)
// 			}
// 			S2C::Login(login) => {
// 				craftflow_protocol::destructure_packet_s2c_login!(
// 					login,
// 					$inner,
// 					($craftflow, $conn_id),
// 				)
// 			}
// 			S2C::Configuration(configuration) => {
// 				craftflow_protocol::destructure_packet_s2c_configuration!(
// 					configuration,
// 					$inner,
// 					($craftflow, $conn_id),
// 				)
// 			}
// 			S2C::Play(play) => {
// 				craftflow_protocol::destructure_packet_s2c_play!(
// 					play,
// 					$inner,
// 					($craftflow, $conn_id),
// 				)
// 			}
// 		};

// 		match r {
// 			Some(r) => r,
// 			None => {
// 				// this means we got an _Unsupported packet
// 				// we are the ones sending the packet so this means this is a retard incident
// 				panic!("DO NOT FUCKING SEND _Unsupported PACKETS!!!!");
// 			}
// 		}
// 	}};
// }

// pub(super) fn trigger_s2c_pre(
// 	craftflow: &CraftFlow,
// 	conn_id: u64,
// 	packet: S2C,
// ) -> ControlFlow<(), S2C> {
// 	fn inner<'a, P: Packet<Direction = S2C> + 'static>(
// 		packet: P,
// 		(craftflow, conn_id): (&CraftFlow, u64),
// 	) -> ControlFlow<(), S2C> {
// 		match craftflow.reactor.event::<P>(&craftflow, (conn_id, packet)) {
// 			ControlFlow::Continue((_conn_id, packet)) => {
// 				ControlFlow::Continue(packet.into_packet_enum())
// 			}
// 			ControlFlow::Break(()) => ControlFlow::Break(()),
// 		}
// 	}

// 	gen_trigger_s2c!(inner, packet, craftflow, conn_id)
// }

// pub(super) fn trigger_s2c_post(
// 	craftflow: &CraftFlow,
// 	conn_id: u64,
// 	packet: S2C,
// ) -> ControlFlow<(), S2C> {
// 	fn inner<'a, P: Packet<Direction = S2C> + 'static>(
// 		packet: P,
// 		(craftflow, conn_id): (&CraftFlow, u64),
// 	) -> ControlFlow<(), S2C> {
// 		match craftflow
// 			.reactor
// 			.event::<Post<P>>(&craftflow, (conn_id, packet))
// 		{
// 			ControlFlow::Continue((_conn_id, packet)) => {
// 				ControlFlow::Continue(packet.into_packet_enum())
// 			}
// 			ControlFlow::Break(()) => ControlFlow::Break(()),
// 		}
// 	}

// 	gen_trigger_s2c!(inner, packet, craftflow, conn_id)
// }
