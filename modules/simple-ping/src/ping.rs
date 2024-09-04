use craftflow::CraftFlow;
use craftflow_protocol::packets::status::{Ping, Pong};
use std::ops::ControlFlow;

pub fn ping(cf: &CraftFlow, (conn_id, request): (usize, Ping)) -> ControlFlow<(), (usize, Ping)> {
	cf.get(conn_id).send(Pong {
		payload: request.payload,
	});

	ControlFlow::Continue((conn_id, request))
}
