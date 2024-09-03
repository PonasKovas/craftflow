use craftflow::CFState;
use craftflow_protocol::packets::status::{Ping, Pong};
use std::ops::ControlFlow;

pub fn ping(cfstate: &mut CFState, (conn_id, request): &mut (usize, Ping)) -> ControlFlow<()> {
	cfstate.connections[*conn_id].send(Pong {
		payload: request.payload,
	});

	ControlFlow::Continue(())
}
