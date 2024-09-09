use craftflow::CraftFlow;
use craftflow_protocol::protocol::{c2s::status::Ping, s2c::status::Pong};
use std::ops::ControlFlow;

pub fn ping(cf: &CraftFlow, (conn_id, request): (usize, Ping)) -> ControlFlow<(), (usize, Ping)> {
	cf.get(conn_id).send(Pong {
		payload: request.payload,
	});

	ControlFlow::Continue((conn_id, request))
}
