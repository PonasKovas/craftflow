use craftflow::{ConnId, CraftFlow, packet_events::Packet};
use craftflow_protocol::{
	c2s::status::Ping,
	disabled_versions,
	s2c::status::{PingBuilder, ping::v5::PingV5},
};
use std::{ops::ControlFlow, sync::Arc};

#[craftflow::callback(event: Packet<Ping>)]
pub async fn ping(
	cf: &Arc<CraftFlow>,
	&mut (conn_id, ref mut request): &mut (ConnId, Ping),
) -> ControlFlow<()> {
	let time = match request {
		Ping::V5(ping) => ping.time,
		disabled_versions!(c2s::status::Ping) => unreachable!(),
	};

	cf.build_packet(conn_id, |b| match b {
		PingBuilder::V5(p) => p(PingV5 { time }),
		disabled_versions!(s2c::status::PingBuilder) => unreachable!(),
	})
	.await;

	ControlFlow::Continue(())
}
