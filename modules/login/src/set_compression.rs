use crate::Login;
use craftflow::packet_events::{Packet, Post};
use craftflow::{ConnId, CraftFlow};
use craftflow_protocol::s2c::login::Compress;
use std::ops::ControlFlow;
use std::sync::Arc;

#[craftflow::callback(event: Post<Packet<Compress>>)]
pub async fn set_compression(
	cf: &Arc<CraftFlow>,
	&mut (conn_id, ref mut _request): &mut (ConnId, Compress),
) -> ControlFlow<()> {
	if let Some(threshold) = cf.modules.get::<Login>().compression_threshold {
		// Enable compression on our end
		cf.get(conn_id).set_compression_threshold(threshold);
	}

	ControlFlow::Continue(())
}
