use crate::SimplePing;
use craftflow::CraftFlow;
use craftflow_protocol_abstract::{
	c2s::AbStatusRequestInfo,
	s2c::{
		status_info::{Players, Version},
		AbStatusInfo,
	},
	MAX_VERSION, MIN_VERSION,
};
use std::ops::ControlFlow;

pub async fn status(
	cf: &CraftFlow,
	&mut (conn_id, ref mut _request): &mut (u64, AbStatusRequestInfo),
) -> ControlFlow<()> {
	let client_protocol_version = cf.get(conn_id).protocol_version();
	let protocol_version =
		if MIN_VERSION <= client_protocol_version && client_protocol_version <= MAX_VERSION {
			client_protocol_version
		} else {
			MIN_VERSION
		};

	let online_players = cf.connections().len() as i32; // more or less
	let max_players = 2_000_000_000; // todo after implementing max connections
	let description = cf.modules.get::<SimplePing>().server_description.clone();
	let favicon = cf.modules.get::<SimplePing>().favicon.clone();

	cf.get(conn_id).send(AbStatusInfo {
		version: Version {
			name: format!("§f§lCraftFlow").into(),
			protocol: protocol_version,
		},
		players: Some(Players {
			max: max_players,
			online: online_players,
			sample: vec![], // todo real player sample
		}),
		description: Some(description),
		favicon,
		enforces_secure_chat: true,
	});

	ControlFlow::Continue(())
}
