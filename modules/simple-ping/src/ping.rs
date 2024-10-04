use craftflow::CraftFlow;
use craftflow_protocol_abstract::{c2s::AbStatusPing, s2c::AbStatusPong};
use std::ops::ControlFlow;

pub fn ping<'a>(
	cf: &'a CraftFlow,
	(conn_id, request): (u64, &'a mut AbStatusPing),
) -> ControlFlow<(), (u64, &'a mut AbStatusPing)> {
	cf.get(conn_id).send(AbStatusPong { id: request.id });

	cf.disconnect(conn_id);

	ControlFlow::Continue((conn_id, request))
}
