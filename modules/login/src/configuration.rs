use crate::Login;
use craftflow::{ConnId, CraftFlow, callback, packet_events::Packet};
use craftflow_protocol::{
	disabled_versions,
	s2c::{
		configuration::{
			FinishConfigurationBuilder, RegistryDataBuilder, SelectKnownPacksBuilder,
			finish_configuration::v764::FinishConfigurationV764,
			registry_data::{
				v764::RegistryDataV764,
				v766::{RegistryDataV766, RegistryEntry},
			},
			select_known_packs::v766::{PackInfo, SelectKnownPacksV766},
		},
		login::Success,
	},
};
use std::{ops::ControlFlow, sync::Arc};

#[callback(event: Packet<Success>)]
pub async fn configuration(
	cf: &Arc<CraftFlow>,
	&mut (conn_id, ref mut _request): &mut (ConnId, Success),
) -> ControlFlow<()> {
	let conn = cf.get(conn_id);

	// known packs
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

	// registry data
	if RegistryDataBuilder::VERSIONS.contains(&conn.protocol_version()) {
		let data = cf.modules.get::<Login>().registry_data.clone();

		match RegistryDataBuilder::new(conn.protocol_version()) {
			RegistryDataBuilder::V764(p) => conn.send(p(RegistryDataV764 { codec: data })).await,
			RegistryDataBuilder::V766(p) => {
				for (key, value) in data.unwrap_compound() {
					conn.send(p(RegistryDataV766 {
						id: key.into_inner(),
						entries: value
							.unwrap_compound()
							.into_iter()
							.map(|(name, val)| RegistryEntry {
								key: name.into_inner(),
								value: Some(val),
							})
							.collect(),
					}))
					.await;
				}
			}
			disabled_versions!(s2c::configuration::RegistryDataBuilder) => unreachable!(),
		}
	}

	cf.build_packet::<FinishConfigurationBuilder>(conn_id, |b| match b {
		FinishConfigurationBuilder::V764(p) => p(FinishConfigurationV764),
		disabled_versions!(s2c::configuration::FinishConfigurationBuilder) => unreachable!(),
	})
	.await;

	ControlFlow::Continue(())
}
