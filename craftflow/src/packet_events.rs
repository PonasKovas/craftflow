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

use crate::ConnId;
use crate::CraftFlow;
use closureslop::Event;
use craftflow_protocol::{C2S, S2C, impl_for};
use std::sync::Arc;
use tracing::trace;

/// Event wrapper for packets
// wrapper because cant implement Event for packets directly bcs neither Event or the packets are defined in this crate
pub struct Packet<E> {
	_packet: E,
}

impl_for! {packet:
impl Event for Packet<packet> {
	type Args<'a> = (ConnId, packet);
	type Return = ();
}}
impl_for! {vpacket:
impl Event for Packet<vpacket> {
	type Args<'a> = (ConnId, vpacket);
	type Return = ();
}}

/// `Post<Packet>` events are emitted after their respective packet events,
/// outgoing packets - after the packet is sent
pub struct Post<E> {
	pub packet: E,
}
impl<E: Event> Event for Post<E> {
	type Args<'a> = E::Args<'a>;
	type Return = E::Return;
}

// Helper functions that trigger a packet event
// returns true if the event was not stopped
async fn helper<'a, P>(craftflow: &Arc<CraftFlow>, conn_id: ConnId, packet: P) -> (bool, P)
where
	Packet<P>: Event<Args<'a> = (ConnId, P)>,
{
	let mut args = (conn_id, packet);

	if craftflow
		.reactor
		.trigger::<Packet<P>>(craftflow, &mut args)
		.await
		.is_break()
	{
		return (false, args.1);
	}

	(true, args.1)
}
async fn helper_post<'a, P>(craftflow: &Arc<CraftFlow>, conn_id: ConnId, packet: P) -> (bool, P)
where
	Packet<P>: Event<Args<'a> = (ConnId, P)>,
{
	let mut args = (conn_id, packet);

	if craftflow
		.reactor
		.trigger::<Post<Packet<P>>>(craftflow, &mut args)
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
	craftflow: &Arc<CraftFlow>,
	conn_id: ConnId,
	packet: C2S,
) -> (bool, C2S) {
	if !post {
		trace!("<- RECV {packet:?}");
	}

	let (cont, pkt) = craftflow_protocol::enum_go_brr!((c2s->version), packet -> inner {
		let (cont, pkt) = if !post { helper(craftflow, conn_id, inner).await } else { helper_post(craftflow, conn_id, inner).await };
		(cont, pkt.into())
	});

	if cont {
		craftflow_protocol::enum_go_brr!((c2s->packet), pkt -> inner {
			let (cont, pkt) = if !post { helper(craftflow, conn_id, inner).await } else { helper_post(craftflow, conn_id, inner).await };
			(cont, pkt.into())
		})
	} else {
		(cont, pkt)
	}
}
pub(super) async fn trigger_s2c(
	post: bool,
	craftflow: &Arc<CraftFlow>,
	conn_id: ConnId,
	packet: S2C,
) -> (bool, S2C) {
	if !post {
		trace!("-> SENT {packet:?}");
	}

	let (cont, pkt) = craftflow_protocol::enum_go_brr!((s2c->version), packet -> inner {
		let (cont, pkt) = if !post { helper(craftflow, conn_id, inner).await } else { helper_post(craftflow, conn_id, inner).await };
		(cont, pkt.into())
	});

	if cont {
		craftflow_protocol::enum_go_brr!((s2c->packet), pkt -> inner {
			let (cont, pkt) = if !post { helper(craftflow, conn_id, inner).await } else { helper_post(craftflow, conn_id, inner).await };
			(cont, pkt.into())
		})
	} else {
		(cont, pkt)
	}
}
