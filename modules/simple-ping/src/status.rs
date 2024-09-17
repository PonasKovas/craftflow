use crate::SimplePing;
use craftflow::CraftFlow;
use craftflow_protocol::{
	protocol::{c2s::status::StatusRequest, s2c::status::StatusResponse},
	serde_types::{
		self,
		status_response::{Players, Version},
	},
};
use std::ops::ControlFlow;

pub fn status(
	cf: &CraftFlow,
	(conn_id, request): (usize, StatusRequest),
) -> ControlFlow<(), (usize, StatusRequest)> {
	let client_protocol_version = cf.get(conn_id).protocol_version();
	let protocol_version = if craftflow_protocol::protocol::SUPPORTED_PROTOCOL_VERSIONS
		.contains(&client_protocol_version)
	{
		client_protocol_version
	} else {
		// just give some random protocol version that we support
		craftflow_protocol::protocol::SUPPORTED_PROTOCOL_VERSIONS[0]
	};

	let online_players = cf.connections().len() as i32; // more or less
	let max_players = 10000; // todo after implementing max connections
	let description = cf.modules.get::<SimplePing>().server_description.clone();
	let favicon = cf.modules.get::<SimplePing>().favicon.clone();

	cf.get(conn_id).send(StatusResponse {
		response: serde_types::status_response::StatusResponse {
			version: Version {
				name: format!("§f§lCraftFlow"),
				protocol: protocol_version as i32,
			},
			players: Some(Players {
				max: max_players,
				online: online_players,
				sample: vec![], // todo real player sample
			}),
			description: Some(description),
			favicon,
			enforces_secure_chat: None,
		},
	});

	ControlFlow::Continue((conn_id, request))
}
