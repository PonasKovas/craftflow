use craftflow::{CraftFlow, callback};
use craftflow_protocol::{
	disabled_versions,
	s2c::{
		configuration::{
			SelectKnownPacksBuilder,
			select_known_packs::v766::{PackInfo, SelectKnownPacksV766},
		},
		login::Success,
	},
};
use std::ops::ControlFlow;

#[callback(event: Success)]
pub async fn send_known_packs(
	cf: &CraftFlow,
	&mut (conn_id, ref mut _request): &mut (u64, Success),
) -> ControlFlow<()> {
	// only needed from 766
	if cf.get(conn_id).protocol_version() < 766 {
		return ControlFlow::Continue(());
	}

	cf.build_packet::<SelectKnownPacksBuilder>(conn_id, |b| match b {
		SelectKnownPacksBuilder::V766(p) => p(SelectKnownPacksV766 {
			packs: vec![PackInfo {
				namespace: "minecraft".into(),
				id: "core".into(),
				version: "1.21.4".into(),
			}],
		}),
		disabled_versions!(s2c::configuration::SelectKnownPacksBuilder) => unreachable!(),
	})
	.await;

	ControlFlow::Continue(())
}
