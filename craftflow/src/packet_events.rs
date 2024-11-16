//! Implementation of `Event` for all packets
//!  - [`C2S`] packet events will be emitted after a concrete packet is received from the client
//!  - [`AbC2S`] packet events will be emitted after an abstract packet is received from the client
//!  - [`S2C`] packet events will be emitted before a concrete packet is sent to the client
//!  - [`AbS2C`] packet events will be emitted before an abstract packet is sent to the client
//!  - [`Post<S2C>`] events will be emitted AFTER a concrete packet is sent to the client
//!  - [`Post<AbS2C>`] events will be emitted AFTER an abstract packet is sent to the client
//!  - [`Post<C2S>`] events will be emitted after the respective [`C2S`] event is over, if it wasn't stopped
//!  - [`Post<AbC2S>`] events will be emitted after the respective [`AbC2S`] event is over, if it wasn't stopped

// BEWARE!
// nuclear code below!!

// This is the slop file that uses macro slop to generate matching and impl blocks slop
// for the purpose of the `Event` trait slop

use crate::{reactor::Event, CraftFlow};
use craftflow_protocol_abstract::{AbC2S, AbS2C};
use craftflow_protocol_versions::{IntoStateEnum, C2S, S2C};
use tracing::trace;

/// `Post<Packet>` events are emitted after their respective packet events,
/// and in the case of outgoing packets - after the packet is sent
pub struct Post<E> {
	pub packet: E,
}
impl<E: Event> Event for Post<E> {
	type Args<'a> = <E as Event>::Args<'a>;
	type Return = <E as Event>::Return;
}

// this is a private trait that helps the macro slop
// the macros implement it for the packet types and the `Event` is the respective packet event
trait PacketToEventPointer<'a> {
	type Event: Event<Args<'a> = (u64, Self)>;
}

// The following macros generate a unit struct for each packet and implements Event for it
craftflow_protocol_versions::__gen_events_for_packets__! {Event, PacketToEventPointer }
craftflow_protocol_abstract::__gen_events_for_packets_s2c! { Event, PacketToEventPointer }
craftflow_protocol_abstract::__gen_events_for_packets_c2s! { Event, PacketToEventPointer }

// Helper functions that trigger a packet event
// returns true if the event was not stopped
fn helper<'a, 'b, P>(craftflow: &'a CraftFlow, conn_id: u64, packet: P) -> (bool, P)
where
	P: PacketToEventPointer<'b>,
{
	trace!(
		"{} event",
		std::any::type_name::<<P as PacketToEventPointer>::Event>()
	);
	let mut args = (conn_id, packet);
	if craftflow
		.reactor
		.event::<<P as PacketToEventPointer>::Event>(craftflow, &mut args)
		.is_break()
	{
		return (false, args.1);
	}
	(true, args.1)
}
fn helper_post<'a, 'b, P>(craftflow: &'a CraftFlow, conn_id: u64, packet: P) -> (bool, P)
where
	P: PacketToEventPointer<'b>,
	Post<<P as PacketToEventPointer<'b>>::Event>: Event<Args<'b> = (u64, P)>,
{
	trace!(
		"{} post event",
		std::any::type_name::<<P as PacketToEventPointer>::Event>()
	);
	let mut args = (conn_id, packet);
	if craftflow
		.reactor
		.event::<Post<<P as PacketToEventPointer>::Event>>(craftflow, &mut args)
		.is_break()
	{
		return (false, args.1);
	}
	(true, args.1)
}

// More macro slop below

pub(super) fn trigger_c2s_concrete<'a, 'b>(
	post: bool,
	craftflow: &'a CraftFlow,
	conn_id: u64,
	packet: C2S<'b>,
) -> (bool, C2S<'b>) {
	craftflow_protocol_versions::__destructure_packet_enum__!(direction=C2S, packet -> inner {
		let (cont, pkt) = if !post { helper(craftflow, conn_id, inner) } else { helper_post(craftflow, conn_id, inner) };
		(cont, pkt.into_state_enum())
	})
}
pub(super) fn trigger_s2c_concrete<'a, 'b>(
	post: bool,
	craftflow: &'a CraftFlow,
	conn_id: u64,
	packet: S2C<'b>,
) -> (bool, S2C<'b>) {
	craftflow_protocol_versions::__destructure_packet_enum__!(direction=S2C, packet -> inner {
		let (cont, pkt) = if !post { helper(craftflow, conn_id, inner) } else { helper_post(craftflow, conn_id, inner) };
		(cont, pkt.into_state_enum())
	})
}
pub(super) fn trigger_c2s_abstract<'a, 'b>(
	post: bool,
	craftflow: &'a CraftFlow,
	conn_id: u64,
	packet: AbC2S<'b>,
) -> (bool, AbC2S<'b>) {
	craftflow_protocol_abstract::__destructure_c2s__!(packet -> inner {
		let (cont, pkt) = if !post { helper(craftflow, conn_id, inner) } else { helper_post(craftflow, conn_id, inner) };
		(cont, pkt.into())
	})
}
pub(super) fn trigger_s2c_abstract<'a, 'b>(
	post: bool,
	craftflow: &'a CraftFlow,
	conn_id: u64,
	packet: AbS2C<'b>,
) -> (bool, AbS2C<'b>) {
	craftflow_protocol_abstract::__destructure_s2c__!(packet -> inner {
		let (cont, pkt) = if !post { helper(craftflow, conn_id, inner) } else { helper_post(craftflow, conn_id, inner) };
		(cont, pkt.into())
	})
}
