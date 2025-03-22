use craftflow::CraftFlow;
use craftflow_protocol::{
	c2s::status::Ping,
	disabled_versions,
	s2c::{self, status::PingBuilder},
};
use std::ops::ControlFlow;

#[craftflow::callback(event: Ping)]
pub async fn ping(
	cf: &CraftFlow,
	&mut (conn_id, ref mut request): &mut (u64, Ping),
) -> ControlFlow<()> {
	let time = match request {
		disabled_versions!(c2s::status::Ping) => unreachable!(),
		Ping::V5(ping) => ping.time,
	};

	let response = match PingBuilder::new(cf.connections()[&conn_id].protocol_version()) {
		disabled_versions!(s2c::status::PingBuilder) => unreachable!(),
		PingBuilder::V5(p) => p(s2c::status::ping::v5::PingV5 { time }),
	};

	cf.get(conn_id).send(response).await;

	ControlFlow::Continue(())
}
