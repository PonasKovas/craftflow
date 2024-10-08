use crate::Login;
use craftflow::CraftFlow;
use craftflow_protocol::protocol::s2c::login::SetCompression;
use std::ops::ControlFlow;

pub fn set_compression(
	cf: &CraftFlow,
	(conn_id, request): (u64, SetCompression),
) -> ControlFlow<(), (u64, SetCompression)> {
	if let &Some(threshold) = &cf.modules.get::<Login>().compression_threshold {
		// Enable compression on our end
		cf.get(conn_id).set_compression_threshold(threshold);
	}

	ControlFlow::Continue((conn_id, request))
}
