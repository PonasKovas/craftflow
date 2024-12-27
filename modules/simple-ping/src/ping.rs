use craftflow::{packet_events::C2SAbStatusPingEvent, CraftFlow};
use craftflow_protocol_abstract::{c2s::AbStatusPing, s2c::AbStatusPong};
use std::ops::ControlFlow;

#[craftflow::callback(event: C2SAbStatusPingEvent)]
pub async fn ping(
	cf: &CraftFlow,
	&mut (conn_id, ref mut request): &mut (u64, AbStatusPing),
) -> ControlFlow<()> {
	cf.get(conn_id).send(AbStatusPong { id: request.id }).await;

	ControlFlow::Continue(())
}
