//! Implementation of `Event` for all packets
//! `C2S` packet events will be emitted after a packet is received from the client
//! `S2C` packet events will be emitted before a packet is sent to the client
//! `Post<S2C>` events will be emitted AFTER a packet is sent to the client

use crate::{
	packets::{C2SPacket, S2CPacket},
	reactor::Event,
	CraftFlow,
};
use craftflow_protocol_versions::{IntoStateEnum, S2C};
use std::{any::Any, marker::PhantomData, ops::ControlFlow};

// All of these Event implementations could have been done without any of this macro slop
// if rust wasnt a retarded language and allowed to specify mutually exclusive traits or negative bounds
// but no,

// Concrete packets
craftflow_protocol_versions::__gen_impls_for_packets__! {
	impl Event for X {
		/// The arguments for this event are the connection ID and the packet
		type Args<'a> = (u64, &'a mut Self);
		/// For S2C packets, if the event is stopped, the packet will not be sent
		type Return = ();
	}
}

// Abstract S2C packets
craftflow_protocol_abstract::__gen_impls_for_packets_s2c! {
	impl Event for X {
		/// The arguments for this event are the connection ID and the packet
		type Args<'a> = (u64, &'a mut Self);
		/// In the case of S2C packets, if the event is stopped, the packet will not be sent
		type Return = ();
	}
}

// Abstract C2S packets
craftflow_protocol_abstract::__gen_impls_for_packets_c2s! {
	impl Event for X {
		/// The arguments for this event are the connection ID and the packet
		type Args<'a> = (u64, &'a mut Self);
		/// In the case of S2C packets, if the event is stopped, the packet will not be sent
		type Return = ();
	}
}

/// `Post<Packet>` events are emitted after a packet is sent to the client
/// Contrary to the normal Packet events, which are emitted before the packet is sent
/// and can modify or stop the packet from being sent
pub struct Post<P: IntoStateEnum<Direction = S2C>> {
	_phantom: PhantomData<P>,
}

impl<P: IntoStateEnum<Direction = S2C> + Any> Event for Post<P> {
	/// The arguments for this event are the connection ID and the packet
	type Args<'a> = (u64, &'a mut P);
	type Return = ();
}

pub(super) fn trigger_c2s(
	craftflow: &CraftFlow,
	conn_id: u64,
	packet: &mut C2SPacket,
) -> ControlFlow<()> {
	fn helper<'a, T>(craftflow: &CraftFlow, conn_id: u64, packet: &mut C2SPacket) -> ControlFlow<()>
	where
		T: Event<Args<'a> = (u64, &'a mut T)>,
	{
		craftflow.trigger_event::<T>(conn_id, packet)
	}

	match packet {
		C2SPacket::Abstract(p) => {
			craftflow_protocol_abstract::__destructure_c2s__!(p -> {
				helper(craftflow, conn_id, p)
			})
		}
		C2SPacket::Concrete(p) => {
			craftflow_protocol_versions::__destructure_packet_enum__!(direction=S2C, p -> {
				helper(craftflow, conn_id, p)
			})
		}
	}
}

pub(super) fn trigger_s2c_pre(
	craftflow: &CraftFlow,
	conn_id: u64,
	packet: &mut S2CPacket,
) -> ControlFlow<()> {
	fn helper<'a, T>(craftflow: &CraftFlow, conn_id: u64, packet: &mut S2CPacket) -> ControlFlow<()>
	where
		T: Event<Args<'a> = (u64, &'a mut T)>,
	{
		craftflow.trigger_event::<T>(conn_id, packet)
	}

	match packet {
		S2CPacket::Abstract(p) => {
			// craftflow_protocol_abstract::__destructure_s2c__!(p -> {
			// 	helper(craftflow, conn_id, p)
			// })
		}
		S2CPacket::Concrete(p) => {
			craftflow_protocol_versions::__destructure_packet_enum__!(direction=S2C, p -> {
				helper(craftflow, conn_id, p)
			})
		}
	}
}

pub(super) fn trigger_s2c_post(
	craftflow: &CraftFlow,
	conn_id: u64,
	packet: &mut S2CPacket,
) -> ControlFlow<()> {
	fn helper<'a, T>(craftflow: &CraftFlow, conn_id: u64, packet: &mut S2CPacket) -> ControlFlow<()>
	where
		T: Event<Args<'a> = (u64, &'a mut T)>,
	{
		craftflow.trigger_event::<Post<T>>(conn_id, packet)
	}

	match packet {
		S2CPacket::Abstract(p) => {
			// craftflow_protocol_abstract::__destructure_s2c__!(p -> {
			// 	helper(craftflow, conn_id, p)
			// })
		}
		S2CPacket::Concrete(p) => {
			craftflow_protocol_versions::__destructure_packet_enum__!(direction=S2C, p -> {
				helper(craftflow, conn_id, p)
			})
		}
	}
}
