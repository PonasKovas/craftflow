use crate::{World, WorldMessage};
use craftflow::various_events::EnterPlayState;
use craftflow::{ConnId, CraftFlow};
use std::ops::ControlFlow;
use std::sync::Arc;

#[craftflow::callback(event: EnterPlayState)]
pub async fn start_play_cb(cf: &Arc<CraftFlow>, &mut conn_id: &mut ConnId) -> ControlFlow<()> {
	let world = cf
		.modules
		.get::<World>()
		.players
		.read()
		.unwrap()
		.get(&conn_id)
		.copied();
	if let Some(world) = world {
		let sender = cf.modules.get::<World>().worlds.read().unwrap()[&world]
			.messages
			.clone();
		sender
			.send(WorldMessage::InitPlayer { conn_id })
			.await
			.unwrap();
	}

	ControlFlow::Continue(())
}
