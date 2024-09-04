use craftflow::CFState;
use craftflow_protocol::packets::status::{Ping, Pong};
use std::ops::ControlFlow;

pub fn ping(
	cfstate: &CFState,
	(conn_id, request): (usize, Ping),
) -> ControlFlow<(), (usize, Ping)> {
	cfstate.connections.get(conn_id).send(Pong {
		payload: request.payload,
	});

	ControlFlow::Continue((conn_id, request))
}
