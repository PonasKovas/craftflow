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

// actually nvm. these comments are from the era when i tried to do packets with lifetimes.
// i trashed that idea now but im gonna keep these comments as a warning.

use crate::CraftFlow;
use closureslop::Event;
use craftflow_protocol::{C2S, S2C};

/// `Post<Packet>` events are emitted after their respective packet events,
/// and in the case of outgoing packets - after the packet is sent
pub struct Post<E> {
	pub packet: E,
}
impl<E: Event> Event for Post<E> {
	type Args<'a> = E::Args<'a>;
	type Return = E::Return;
}

// Helper functions that trigger a packet event
// returns true if the event was not stopped
async fn helper<'a, P>(craftflow: &CraftFlow, conn_id: u64, packet: P) -> (bool, P)
where
	P: Event<Args<'a> = (u64, P)>,
{
	let mut args = (conn_id, packet);

	if craftflow
		.reactor
		.trigger::<P>(craftflow, &mut args)
		.await
		.is_break()
	{
		return (false, args.1);
	}

	(true, args.1)
}
async fn helper_post<'a, P>(craftflow: &CraftFlow, conn_id: u64, packet: P) -> (bool, P)
where
	P: Event<Args<'a> = (u64, P)>,
{
	let mut args = (conn_id, packet);

	if craftflow
		.reactor
		.trigger::<Post<P>>(craftflow, &mut args)
		.await
		.is_break()
	{
		return (false, args.1);
	}

	(true, args.1)
}

// More slop below

pub(super) async fn trigger_c2s(
	post: bool,
	craftflow: &CraftFlow,
	conn_id: u64,
	packet: C2S,
) -> (bool, C2S) {
	craftflow_protocol::enum_go_brr!((c2s->version), packet -> inner {
		let (cont, pkt) = if !post { helper(craftflow, conn_id, inner).await } else { helper_post(craftflow, conn_id, inner).await };
		(cont, pkt.into())
	})
}
pub(super) async fn trigger_s2c(
	post: bool,
	craftflow: &CraftFlow,
	conn_id: u64,
	packet: S2C,
) -> (bool, S2C) {
	craftflow_protocol::enum_go_brr!((s2c->version), packet -> inner {
		let (cont, pkt) = if !post { helper(craftflow, conn_id, inner).await } else { helper_post(craftflow, conn_id, inner).await };
		(cont, pkt.into())
	})
}
