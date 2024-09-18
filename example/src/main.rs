use std::ops::ControlFlow;

use craftflow::CraftFlow;
use craftflow_protocol::{
	datatypes::{nbt, VarInt},
	protocol::{
		c2s::{configuration::AcknowledgeFinishConfiguration, login::LoginAcknowledged},
		s2c::{
			configuration::{FinishConfiguration, RegistryData, RegistryEntry},
			play,
		},
	},
	text,
};
use login::Login;
use simple_ping::SimplePing;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	tracing_subscriber::fmt()
		// ... add configuration
		.init();

	let mut craftflow = CraftFlow::new();

	SimplePing::new()
		.set_description(
			text!("This is an example server! ", color = "gold")
				+ text!("CONNECT ", bold, color = "aqua")
				+ text!("NOW!", bold, color = "green"),
		)
		.register(&mut craftflow);

	Login::default().register(&mut craftflow);

	craftflow
		.reactor
		.add_handler::<LoginAcknowledged, _>(|cf, (conn_id, packet)| {
			cf.get(conn_id).send(RegistryData {
				registry_id: format!("dimension_type"),
				entries: vec![RegistryEntry {
					entry_id: format!("craftflow_dimension"),
					data: Some(nbt!("", {
						"fixed_time": 0i64,
						"has_skylight": false,
						"has_ceiling": false,
						"ultrawarm": false,
						"natural": false,
						"coordinate_scale": 1f64,
						"bed_works": false,
						"respawn_anchor_works": false,
						"min_y": 0i32,
						"height": 512i32,
						"logical_height": 512i32,
						"infiniburn": "#",
						"effects": "the_end",
						"ambient_light": 0f32,
						"piglin_safe": false,
						"has_raids": false,
						"monster_spawn_light_level": 0i32,
						"monster_spawn_block_light_limit": 0i32,
					})),
				}],
			});
			cf.get(conn_id).send(FinishConfiguration {});

			ControlFlow::Continue((conn_id, packet))
		});

	craftflow
		.reactor
		.add_handler::<AcknowledgeFinishConfiguration, _>(|cf, (conn_id, packet)| {
			cf.get(conn_id).send(play::Login {
				entity_id: 0,
				is_hardcore: false,
				dimension_names: vec![format!("craftflow_dimension")],
				max_players: VarInt(0),
				view_distance: VarInt(12),
				simulation_distance: VarInt(12),
				reduced_debug_info: false,
				enable_respawn_screen: false,
				limited_crafting: false,
				dimension_type: VarInt(0),
				dimension_name: format!("craftflow_dimension"),
				hashed_seed: 0,
				game_mode: play::GameMode::Creative {},
				previous_game_mode: play::GameMode::Creative {},
				is_debug_world: false,
				is_flat_world: true,
				death_location: None,
				portal_cooldown: VarInt(0),
				enforces_secure_chat: false,
			});

			ControlFlow::Continue((conn_id, packet))
		});

	craftflow.run().await
}
