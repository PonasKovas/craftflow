#![doc(
	html_favicon_url = "https://github.com/PonasKovas/craftflow/blob/master/assets/icon.png?raw=true"
)]
#![doc(
	html_logo_url = "https://github.com/PonasKovas/craftflow/blob/master/assets/icon.png?raw=true"
)]
// used for Craftflow::get to get access to a connection through a lock
#![feature(mapped_lock_guards)]

pub mod connection;
pub mod modules;
pub mod packet_events;
pub mod various_events;

use closureslop::Reactor;
use connection::{new_conn_interface, ConnectionInterface};
use modules::Modules;
use std::{
	collections::HashMap,
	sync::{Arc, MappedRwLockReadGuard, RwLock, RwLockReadGuard},
};
use tokio::net::TcpListener;
use various_events::{Disconnect, NewConnection};

pub struct CraftFlow {
	connections: RwLock<Connections>,
	pub modules: Modules,
	pub reactor: Reactor<CraftFlow>,
}

struct Connections {
	connections: HashMap<u64, ConnectionInterface>,
	next_conn_id: u64,
}

impl CraftFlow {
	pub fn new() -> Self {
		Self {
			connections: RwLock::new(Connections {
				connections: HashMap::new(),
				next_conn_id: 0,
			}),
			modules: Modules::new(),
			reactor: Reactor::new(),
		}
	}

	/// Runs the CraftFlow server
	pub async fn run(self) -> anyhow::Result<()> {
		let craftflow = Arc::new(self);

		// Start accepting connections in this task
		let listener = TcpListener::bind("0.0.0.0:25565").await?;

		loop {
			let (stream, _) = listener.accept().await?;

			let id = new_conn_interface(&craftflow, stream);

			// Emit the new connection event
			if craftflow
				.reactor
				.trigger::<NewConnection>(&craftflow, &mut id.clone())
				.await
				.is_break()
			{
				// immediately disconnect the client
				craftflow.disconnect(id);
			}
		}
	}
	/// Accesses the connection handle of the given connection ID
	pub fn get<'a>(&'a self, conn_id: u64) -> MappedRwLockReadGuard<'a, ConnectionInterface> {
		RwLockReadGuard::map(self.connections.read().unwrap(), |inner| {
			&inner.connections[&conn_id]
		})
	}
	/// Disconnects the client with the given connection ID
	/// No-op if the client is already disconnected, panic if the client ID was never connected
	pub fn disconnect(&self, conn_id: u64) {
		let mut connections = self.connections.write().unwrap();

		if connections.next_conn_id <= conn_id {
			panic!("attempt to disconnect client with ID {conn_id} that was never connected");
		}

		if connections.connections.contains_key(&conn_id) {
			// emit the disconnect event
			self.reactor
				.event::<Disconnect>(&self, &mut conn_id.clone());
		}

		connections.connections.remove(&conn_id);
	}
	/// Accesses the connections map
	/// There is no mutable access because it is not meant to be modified directly
	/// Use the `disconnect` method to disconnect a client
	pub fn connections<'a>(
		&'a self,
	) -> MappedRwLockReadGuard<'a, HashMap<u64, ConnectionInterface>> {
		RwLockReadGuard::map(self.connections.read().unwrap(), |inner| &inner.connections)
	}
}
