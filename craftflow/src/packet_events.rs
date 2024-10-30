//! Implementation of `Event` for all packets
//!  - [`C2S`] packet events will be emitted after a concrete packet is received from the client
//!  - [`AbC2S`] packet events will be emitted after an abstract packet is received from the client
//!  - [`S2C`] packet events will be emitted before a concrete packet is sent to the client
//!  - [`AbS2C`] packet events will be emitted before an abstract packet is sent to the client
//!  - [`Post<S2C>`] events will be emitted AFTER a concrete packet is sent to the client
//!  - [`Post<AbS2C>`] events will be emitted AFTER an abstract packet is sent to the client

// This is the slop file that uses macro slop to generate matching and impl blocks slop
// for the purpose of the `Event` trait slop

use crate::{reactor::Event, CraftFlow};
use craftflow_protocol_abstract::{AbC2S, AbS2C};
use craftflow_protocol_versions::{C2S, S2C};
use std::ops::ControlFlow;
use tracing::trace;

// All of these Event implementations could have been done without any of this macro slop
// if rust wasnt a retarded language and allowed to specify mutually exclusive traits or negative bounds
// but no,

// Concrete packets
craftflow_protocol_versions::__gen_impls_for_packets__! {
	impl Event for X {
		/// The arguments for this event are the connection ID and the packet
		type Args<'a> = (u64, &'a mut X);
		/// For S2C packets, if the event is stopped, the packet will not be sent
		type Return = ();
	}
}

// Abstract S2C packets
craftflow_protocol_abstract::__gen_impls_for_packets_s2c! {
	impl Event for X {
		/// The arguments for this event are the connection ID and the packet
		type Args<'a> = (u64, &'a mut X);
		/// In the case of S2C packets, if the event is stopped, the packet will not be sent
		type Return = ();
	}
}

// Abstract C2S packets
craftflow_protocol_abstract::__gen_impls_for_packets_c2s! {
	impl Event for X {
		/// The arguments for this event are the connection ID and the packet
		type Args<'a> = (u64, &'a mut X);
		/// In the case of S2C packets, if the event is stopped, the packet will not be sent
		type Return = ();
	}
}

/// `Post<Packet>` events are emitted after a packet is sent to the client
/// Contrary to the normal Packet events, which are emitted before the packet is sent
/// and can modify or stop the packet from being sent
pub struct Post<P> {
	pub packet: P,
}

// POST Concrete packets
craftflow_protocol_versions::__gen_impls_for_packets__! {
	impl Event for Post<X> {
		/// The arguments for this event are the connection ID and the packet
		type Args<'a> = (u64, &'a mut X);
		/// For S2C packets, if the event is stopped, the packet will not be sent
		type Return = ();
	}
}

// POST Abstract packets
craftflow_protocol_abstract::__gen_impls_for_packets_s2c! {
	impl Event for Post<X> {
		/// The arguments for this event are the connection ID and the packet
		type Args<'a> = (u64, &'a mut X);
		/// In the case of S2C packets, if the event is stopped, the packet will not be sent
		type Return = ();
	}
}

// Helper function that triggers a packet event
fn helper<'a, E>(
	craftflow: &'a CraftFlow,
	conn_id: u64,
	packet: &'a mut E,
) -> ControlFlow<E::Return>
where
	E: Event<Args<'a> = (u64, &'a mut E)>,
{
	trace!("{} event, ", std::any::type_name::<E>());
	craftflow.reactor.event::<E>(craftflow, (conn_id, packet))?;
	ControlFlow::Continue(())
}

// More macro slop below

pub(super) fn trigger_c2s_concrete(
	craftflow: &CraftFlow,
	conn_id: u64,
	packet: &mut C2S,
) -> ControlFlow<()> {
	craftflow_protocol_versions::__destructure_packet_enum__!(direction=C2S, packet -> inner {
		helper(craftflow, conn_id, inner)
	})
}
pub(super) fn trigger_c2s_abstract(
	craftflow: &CraftFlow,
	conn_id: u64,
	packet: &mut AbC2S,
) -> ControlFlow<()> {
	craftflow_protocol_abstract::__destructure_c2s__!(packet -> inner {
		helper(craftflow, conn_id, inner)
	})
}

pub(super) fn trigger_s2c_concrete_pre(
	craftflow: &CraftFlow,
	conn_id: u64,
	packet: &mut S2C,
) -> ControlFlow<()> {
	craftflow_protocol_versions::__destructure_packet_enum__!(direction=S2C, packet -> inner {
		helper(craftflow, conn_id, inner)
	})
}
pub(super) fn trigger_s2c_abstract_pre(
	craftflow: &CraftFlow,
	conn_id: u64,
	packet: &mut AbS2C,
) -> ControlFlow<()> {
	craftflow_protocol_abstract::__destructure_s2c__!(packet -> inner {
		helper(craftflow, conn_id, inner)
	})
}

// Special helper fn for POST events
fn helper_post<'a, E>(
	craftflow: &'a CraftFlow,
	conn_id: u64,
	packet: &'a mut E,
) -> ControlFlow<<Post<E> as Event>::Return>
where
	Post<E>: Event<Args<'a> = (u64, &'a mut E)>,
{
	trace!("{} event, ", std::any::type_name::<Post<E>>());
	craftflow
		.reactor
		.event::<Post<E>>(craftflow, (conn_id, packet))?;
	ControlFlow::Continue(())
}

pub(super) fn trigger_s2c_concrete_post(
	craftflow: &CraftFlow,
	conn_id: u64,
	packet: &mut S2C,
) -> ControlFlow<()> {
	craftflow_protocol_versions::__destructure_packet_enum__!(direction=S2C, packet -> inner {
		helper_post(craftflow, conn_id, inner)
	})
}

pub(super) fn trigger_s2c_abstract_post(
	craftflow: &CraftFlow,
	conn_id: u64,
	packet: &mut AbS2C,
) -> ControlFlow<()> {
	craftflow_protocol_abstract::__destructure_s2c__!(packet -> inner {
		helper_post(craftflow, conn_id, inner)
	})
}
