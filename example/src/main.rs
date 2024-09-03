use std::ops::ControlFlow;

use base64::Engine;
use craftflow::CraftFlow;
use craftflow_protocol::packets::{
	legacy::{LegacyPing, LegacyPingResponse},
	status::{
		Ping, PlayerSample, Players, Pong, StatusRequest, StatusResponse, StatusResponseJSON,
		Version,
	},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let subscriber = tracing_subscriber::fmt()
		// ... add configuration
		.init();

	let mut craftflow = CraftFlow::new();

	craftflow
		.reactor
		.add_handler::<LegacyPing, _>(|cfstate, (conn_id, _)| {
			cfstate.connections[*conn_id].send(
				LegacyPingResponse::new(127, 0, 1000)
					.set_description(format!("§6§lCraftFlow"))
					.set_version(format!("§6§lCraftFlow")),
			);

			ControlFlow::Continue(())
		});

	craftflow
		.reactor
		.add_handler::<StatusRequest, _>(|cfstate, (conn_id, request)| {
			cfstate.connections[*conn_id].send(StatusResponse {
				response: StatusResponseJSON {
					version: Version {
						name: format!("§6§lCraftFlow"),
						protocol: cfstate.connections[*conn_id].protocol_version(),
					},
					players: Some(Players {
						max: 1000,
						online: 0,
						sample: vec![PlayerSample {
							name: format!("§c§lponas"),
							id: format!("00000000-0000-0000-0000-000000000000"),
						}],
					}),
					description: serde_json::from_value(serde_json::json! {
						"§6§lCraftFlow"
					})
					.unwrap(),
					favicon: Some(format!(
						"data:image/png;base64,{}",
						base64::prelude::BASE64_STANDARD
							.encode(include_bytes!("../../assets/icon.png"))
					)),
					enforces_secure_chat: false,
				},
			});

			ControlFlow::Continue(())
		});

	craftflow
		.reactor
		.add_handler::<Ping, _>(|cfstate, (conn_id, request)| {
			cfstate.connections[*conn_id].send(Pong {
				payload: request.payload,
			});

			ControlFlow::Continue(())
		});

	craftflow.run().await
}
