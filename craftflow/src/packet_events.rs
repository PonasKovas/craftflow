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
use craftflow_protocol_versions::{C2S, S2C};
use std::ops::ControlFlow;
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
trait PacketToEventPointer<'a>
where
	Self: 'a,
{
	type Event: Event<Args<'a> = (u64, &'a mut Self)>;
}

// The following macros generate a unit struct for each packet and implements Event for it
craftflow_protocol_versions::__gen_events_for_packets__! {Event, PacketToEventPointer }
craftflow_protocol_abstract::__gen_events_for_packets_s2c! { Event, PacketToEventPointer }
craftflow_protocol_abstract::__gen_events_for_packets_c2s! { Event, PacketToEventPointer }

// Helper functions that trigger a packet event
fn helper<'a, 'b: 'a, P>(
	craftflow: &'a CraftFlow,
	conn_id: u64,
	packet: &'a mut P,
) -> ControlFlow<<<P as PacketToEventPointer<'b>>::Event as Event>::Return>
where
	P: PacketToEventPointer<'b>,
{
	trace!(
		"{} event",
		std::any::type_name::<<P as PacketToEventPointer>::Event>()
	);
	craftflow
		.reactor
		.event::<<P as PacketToEventPointer>::Event>(craftflow, (conn_id, packet))?;
	ControlFlow::Continue(())
}
fn helper_post<'a, P>(
	craftflow: &'a CraftFlow,
	conn_id: u64,
	packet: &'a mut P,
) -> ControlFlow<<Post<<P as PacketToEventPointer<'a>>::Event> as Event>::Return>
where
	P: PacketToEventPointer<'a>,
	Post<<P as PacketToEventPointer<'a>>::Event>: Event<Args<'a> = (u64, &'a mut P)>,
{
	trace!(
		"{} post event",
		std::any::type_name::<<P as PacketToEventPointer>::Event>()
	);
	craftflow
		.reactor
		.event::<Post<<P as PacketToEventPointer>::Event>>(craftflow, (conn_id, packet))?;
	ControlFlow::Continue(())
}

// More macro slop below

pub(super) fn trigger_c2s_concrete<'a, 'b: 'a>(
	post: bool,
	craftflow: &'a CraftFlow,
	conn_id: u64,
	packet: &'a mut C2S<'b>,
) -> ControlFlow<()> {
	craftflow_protocol_versions::__destructure_packet_enum__!(direction=C2S, packet -> inner {
		if !post { helper(craftflow, conn_id, inner) } else { helper_post(craftflow, conn_id, inner) }
	})
}
pub(super) fn trigger_s2c_concrete<'a>(
	post: bool,
	craftflow: &'a CraftFlow,
	conn_id: u64,
	packet: &'a mut S2C<'a>,
) -> ControlFlow<()> {
	craftflow_protocol_versions::__destructure_packet_enum__!(direction=S2C, packet -> inner {
		if !post { helper(craftflow, conn_id, inner) } else { helper_post(craftflow, conn_id, inner) }
	})
}
pub(super) fn trigger_c2s_abstract<'a>(
	post: bool,
	craftflow: &'a CraftFlow,
	conn_id: u64,
	packet: &'a mut AbC2S<'a>,
) -> ControlFlow<()> {
	craftflow_protocol_abstract::__destructure_c2s__!(packet -> inner {
		if !post { helper(craftflow, conn_id, inner) } else { helper_post(craftflow, conn_id, inner) }
	})
}
pub(super) fn trigger_s2c_abstract<'a>(
	post: bool,
	craftflow: &'a CraftFlow,
	conn_id: u64,
	packet: &'a mut AbS2C<'a>,
) -> ControlFlow<()> {
	craftflow_protocol_abstract::__destructure_s2c__!(packet -> inner {
		if !post { helper(craftflow, conn_id, inner) } else { helper_post(craftflow, conn_id, inner) }
	})
}
