use craftflow::CraftFlow;
use craftflow_protocol::protocol::{c2s::status::Ping, s2c::status::Pong};
use std::ops::ControlFlow;

pub fn ping(cf: &CraftFlow, (conn_id, request): (u64, Ping)) -> ControlFlow<(), (u64, Ping)> {
	cf.get(conn_id).send(Pong {
		payload: request.payload,
	});

	cf.disconnect(conn_id);

	ControlFlow::Continue((conn_id, request))
}
