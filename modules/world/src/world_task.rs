use std::sync::Arc;

use crate::{WorldId, WorldMessage};
use craftflow::CraftFlow;
use craftflow_protocol::{
	disabled_versions,
	s2c::play::{
		Login, LoginBuilder,
		login::{v5::LoginV5, v47::LoginV47},
	},
};
use tokio::sync::mpsc::Receiver;

struct WorldState {
	cf: Arc<CraftFlow>,
	players: Vec<u64>,
}

pub async fn world_task(cf: Arc<CraftFlow>, id: WorldId, mut receiver: Receiver<WorldMessage>) {
	let mut state = WorldState {
		cf,
		players: Vec::new(),
	};

	while let Some(msg) = receiver.recv().await {
		match msg {
			WorldMessage::InitPlayer { conn_id } => {
				state.players.push(conn_id);
				println!("added player {conn_id} to world {id:?}");

				// :(
				state
					.cf
					.build_packet::<LoginBuilder>(conn_id, |b| match b {
						LoginBuilder::V5(p) => p(LoginV5 {
							entity_id: 0,
							game_mode: 0,
							dimension: 0,
							difficulty: 0,
							max_players: 1,
							level_type: "Test".to_string(),
						}),
						LoginBuilder::V47(p) => p(LoginV47 {
							entity_id: 0,
							game_mode: 0,
							dimension: 0,
							difficulty: 0,
							max_players: 1,
							level_type: "Test".to_string(),
							reduced_debug_info: false,
						}),
						LoginBuilder::V109(p) => todo!(),
						LoginBuilder::V477(p) => todo!(),
						LoginBuilder::V573(p) => todo!(),
						LoginBuilder::V735(p) => todo!(),
						LoginBuilder::V751(p) => todo!(),
						LoginBuilder::V755(p) => todo!(),
						LoginBuilder::V757(p) => todo!(),
						LoginBuilder::V759(p) => todo!(),
						LoginBuilder::V763(p) => todo!(),
						LoginBuilder::V764(p) => todo!(),
						LoginBuilder::V766(p) => todo!(),
						LoginBuilder::V768(p) => todo!(),
						disabled_versions!(s2c::play::LoginBuilder) => unreachable!(),
					})
					.await;
			}
		}
	}
}
