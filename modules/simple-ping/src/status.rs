use crate::SimplePing;
use craftflow::CFState;
use craftflow_protocol::packets::status::{
	Players, StatusRequest, StatusResponse, StatusResponseJSON, Version,
};
use std::ops::ControlFlow;

pub fn status(
	cfstate: &CFState,
	(conn_id, request): (usize, StatusRequest),
) -> ControlFlow<(), (usize, StatusRequest)> {
	let protocol_version = cfstate.connections.get(conn_id).protocol_version(); // TODO send real protocol version i guess
	let online_players = cfstate.connections.read().unwrap().len() as i32; // more or less
	let max_players = 1000; // todo after implementing max connections
	let description = &cfstate.modules.get::<SimplePing>().server_description;
	let favicon = &cfstate.modules.get::<SimplePing>().favicon;

	cfstate.connections.get(conn_id).send(StatusResponse {
		response: StatusResponseJSON {
			version: Version {
				name: format!("§f§lCraftFlow"),
				protocol: protocol_version,
			},
			players: Some(Players {
				max: max_players,
				online: online_players,
				sample: vec![], // todo real player sample
			}),
			description: description.clone(),
			favicon: favicon.clone(),
			enforces_secure_chat: false,
		},
	});

	ControlFlow::Continue((conn_id, request))
}
