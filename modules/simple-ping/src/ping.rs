use craftflow::CraftFlow;
use craftflow_protocol_abstract::{c2s::AbStatusPing, s2c::AbStatusPong};
use std::ops::ControlFlow;

pub fn ping(cf: &CraftFlow, (conn_id, request): &mut (u64, AbStatusPing)) -> ControlFlow<()> {
	cf.get(*conn_id).send(AbStatusPong { id: request.id });

	ControlFlow::Continue(())
}
