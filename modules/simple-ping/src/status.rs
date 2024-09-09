use crate::SimplePing;
use craftflow::CraftFlow;
use craftflow_protocol::protocol::{c2s::status::StatusRequest, s2c::status::StatusResponse};
use serde_json::json;
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
	let description = &cf.modules.get::<SimplePing>().server_description;
	let favicon = &cf.modules.get::<SimplePing>().favicon;

	cf.get(conn_id).send(StatusResponse {
		json_response: serde_json::to_string(&json!({
			"version": {
				"name": format!("§f§lCraftFlow"),
				"protocol": protocol_version,
			},
			"players": {
				"max": max_players,
				"online": online_players,
				"sample": [], // todo real player sample
			},
			"description": description,
			"favicon": favicon,
			"enforces_secure_chat": false
		}))
		.unwrap(),
	});

	ControlFlow::Continue((conn_id, request))
}
