use crate::Login;
use craftflow::{packet_events::Post, CraftFlow};
use craftflow_protocol_abstract::s2c::AbLoginCompress;
use std::ops::ControlFlow;

pub fn set_compression<'a>(
	cf: &CraftFlow,
	(conn_id, request): (u64, &'a mut Post<AbLoginCompress>),
) -> ControlFlow<(), (u64, &'a mut Post<AbLoginCompress>)> {
	if let &Some(threshold) = &cf.modules.get::<Login>().compression_threshold {
		// Enable compression on our end
		cf.get(conn_id).set_compression_threshold(threshold);
	}

	ControlFlow::Continue((conn_id, request))
}
