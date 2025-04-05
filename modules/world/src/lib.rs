#![doc(
	html_favicon_url = "https://github.com/PonasKovas/craftflow/blob/master/assets/icon.png?raw=true"
)]
#![doc(
	html_logo_url = "https://github.com/PonasKovas/craftflow/blob/master/assets/icon.png?raw=true"
)]

mod login_play;
mod world_task;

use craftflow::{ConnId, CraftFlow, callback, connection::State, various_events::Init};
use std::{
	collections::HashMap,
	ops::ControlFlow,
	sync::{
		Arc, OnceLock, RwLock, Weak,
		atomic::{AtomicU64, Ordering},
	},
};
use tokio::{
	spawn,
	sync::mpsc::{Sender, channel},
};
use world_task::world_task;

craftflow::init!();

const CHANNEL_SIZE: usize = 64;

/// A module that handles worlds
pub struct World {
	craftflow: OnceLock<Weak<CraftFlow>>,
	worlds: RwLock<HashMap<WorldId, WorldInstance>>,
	id_counter: AtomicU64,
	players: RwLock<HashMap<ConnId, WorldId>>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
pub struct WorldId(u64);

#[derive(Debug, Clone)]
pub struct WorldInstance {
	messages: Sender<WorldMessage>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WorldMessage {
	/// Sent when a client is in this world and in the Play state
	InitPlayer { conn_id: ConnId },
}

impl World {
	fn cf(&self) -> Arc<CraftFlow> {
		self.craftflow.get().unwrap().upgrade().unwrap()
	}
	pub fn new() -> Self {
		Self {
			craftflow: OnceLock::new(),
			worlds: RwLock::new(HashMap::new()),
			id_counter: AtomicU64::new(0),
			players: RwLock::new(HashMap::new()),
		}
	}
	/// Adds the module to a CraftFlow instance.
	pub fn register(self, craftflow: &mut CraftFlow) {
		craftflow.modules.register(self);

		craftflow::reg!(to: &mut craftflow.reactor);
	}

	pub fn add_world(&self) -> WorldId {
		let new_id = WorldId(self.id_counter.fetch_add(1, Ordering::Relaxed));
		let (sender, receiver) = channel(CHANNEL_SIZE);

		spawn(world_task(self.cf(), new_id, receiver));

		self.worlds
			.write()
			.unwrap()
			.insert(new_id, WorldInstance { messages: sender });

		new_id
	}
	pub async fn set_player(&self, id: ConnId, world_id: WorldId) {
		self.players
			.write()
			.unwrap()
			.entry(id)
			.insert_entry(world_id);

		// if player already in Play state, init immediatelly,
		// otherwise will be initialised when entering play state
		if self.cf().get(id).state() == State::Play {
			let sender = self.worlds.read().unwrap()[&world_id].messages.clone();
			sender
				.send(WorldMessage::InitPlayer { conn_id: id })
				.await
				.unwrap();
		}
	}
}

#[callback(event: Init)]
async fn init(cf: &Arc<CraftFlow>, _: &mut ()) -> ControlFlow<String> {
	cf.modules
		.get::<World>()
		.craftflow
		.set(Arc::downgrade(cf))
		.unwrap();

	ControlFlow::Continue(())
}
