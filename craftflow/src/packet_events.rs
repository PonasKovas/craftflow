//! Implementation of `Event` for all packets
//!  - [`C2S`] packet events will be emitted after a packet is received from the client
//!  - [`S2C`] packet events will be emitted before a packet is sent to the client
//!  - [`Post<S2C>`] events will be emitted AFTER a packet is sent to the client
//!  - [`Post<C2S>`] events will be emitted after the respective [`C2S`] event is over, if it wasn't stopped

// BEWARE!
// nuclear code below!!
//
// 27 compiler bugs have been found while writing this code
// 14 twisted workarounds scattered here
// 500+ hours of debugging
// 9 mental breakdowns
// 1 existential crisis

// This is the slop file that uses macro slop to generate trait slop and pattern matching slop
// for the purpose of the `Event` slop

use crate::CraftFlow;
use closureslop::Event;
use is_type::Is;
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

// Helper functions that trigger a packet event
// returns true if the event was not stopped
async fn helper<'a, P>(craftflow: &CraftFlow, conn_id: u64, packet: P) -> (bool, P)
where
	P: PacketToEventPointer,
	<P as PacketToEventPointer>::Event: Event,
	for<'b> <<P as PacketToEventPointer>::Event as Event>::Args<'b>: Send,
	// no idea why we cant just specify that P::Event: Event<Args<'a> = (u64, P)>
	// but this works...
	(u64, P): Is<Type = <<P as PacketToEventPointer>::Event as Event>::Args<'a>>,
	<<P as PacketToEventPointer>::Event as Event>::Args<'a>: Is<Type = (u64, P)>,
{
	trace!(
		"{} event",
		std::any::type_name::<<P as PacketToEventPointer>::Event>()
	);
	let mut args = (conn_id, packet).into_val();

	if craftflow
		.reactor
		.trigger::<<P as PacketToEventPointer>::Event>(craftflow, &mut args)
		.await
		.is_break()
	{
		return (false, args.into_val().1);
	}
	(true, args.into_val().1)
}
async fn helper_post<'a, P>(craftflow: &CraftFlow, conn_id: u64, packet: P) -> (bool, P)
where
	P: PacketToEventPointer,
	Post<<P as PacketToEventPointer>::Event>: Event,
	for<'b> <Post<<P as PacketToEventPointer>::Event> as Event>::Args<'b>: Send,
	// no idea why we cant just specify that P::Event: Event<Args<'a> = (u64, P)>
	// but this works...
	(u64, P): Is<Type = <Post<<P as PacketToEventPointer>::Event> as Event>::Args<'a>>,
	<Post<<P as PacketToEventPointer>::Event> as Event>::Args<'a>: Is<Type = (u64, P)>,
{
	trace!(
		"{} post event",
		std::any::type_name::<<P as PacketToEventPointer>::Event>()
	);
	let mut args = (conn_id, packet).into_val();
	if craftflow
		.reactor
		.trigger::<Post<<P as PacketToEventPointer>::Event>>(craftflow, &mut args)
		.await
		.is_break()
	{
		return (false, args.into_val().1);
	}
	(true, args.into_val().1)
}

// More slop below

pub(super) async fn trigger_c2s_concrete<'a>(
	post: bool,
	craftflow: &CraftFlow,
	conn_id: u64,
	packet: C2S<'a>,
) -> (bool, C2S<'a>) {
	craftflow_protocol_versions::__destructure_packet_enum__!(direction=C2S, packet -> inner {
		let (cont, pkt) = if !post { helper(craftflow, conn_id, inner).await } else { helper_post(craftflow, conn_id, inner).await };
		(cont, pkt.into_state_enum())
	})
}
pub(super) async fn trigger_s2c_concrete<'a, 'b>(
	post: bool,
	craftflow: &'a CraftFlow,
	conn_id: u64,
	packet: S2C<'b>,
) -> (bool, S2C<'b>) {
	craftflow_protocol_versions::__destructure_packet_enum__!(direction=S2C, packet -> inner {
		let (cont, pkt) = if !post { helper(craftflow, conn_id, inner).await } else { helper_post(craftflow, conn_id, inner).await };
		(cont, pkt.into_state_enum())
	})
}
pub(super) async fn trigger_c2s_abstract<'a, 'b>(
	post: bool,
	craftflow: &'a CraftFlow,
	conn_id: u64,
	packet: AbC2S<'b>,
) -> (bool, AbC2S<'b>) {
	craftflow_protocol_abstract::__destructure_c2s__!(packet -> inner {
		let (cont, pkt) = if !post { helper(craftflow, conn_id, inner).await } else { helper_post(craftflow, conn_id, inner).await };
		(cont, pkt.into())
	})
}
pub(super) async fn trigger_s2c_abstract<'a, 'b>(
	post: bool,
	craftflow: &'a CraftFlow,
	conn_id: u64,
	packet: AbS2C<'b>,
) -> (bool, AbS2C<'b>) {
	craftflow_protocol_abstract::__destructure_s2c__!(packet -> inner {
		let (cont, pkt) = if !post { helper(craftflow, conn_id, inner).await } else { helper_post(craftflow, conn_id, inner).await };
		(cont, pkt.into())
	})
}
