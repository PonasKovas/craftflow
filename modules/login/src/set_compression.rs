use crate::Login;
use craftflow::packet_events::{Post, S2CAbLoginCompressEvent};
use craftflow::CraftFlow;
use craftflow_protocol_abstract::s2c::AbLoginCompress;
use std::ops::ControlFlow;

#[craftflow::callback(event: Post<S2CAbLoginCompressEvent>)]
pub async fn set_compression(
	cf: &CraftFlow,
	&mut (conn_id, ref mut _request): &mut (u64, AbLoginCompress),
) -> ControlFlow<()> {
	if let &Some(threshold) = &cf.modules.get::<Login>().compression_threshold {
		// Enable compression on our end
		cf.get(conn_id).set_compression_threshold(threshold);
	}

	ControlFlow::Continue(())
}
