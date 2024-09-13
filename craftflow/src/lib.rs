#![feature(mapped_lock_guards)]

pub mod connection_handle;
pub mod modules;
// pub mod packet_events;
pub mod reactor;
pub mod various_events;

use connection_handle::ConnectionHandle;
use modules::Modules;
use reactor::Reactor;
use slab::Slab;
use std::sync::{Arc, MappedRwLockReadGuard, RwLock, RwLockReadGuard};
use tokio::net::TcpListener;
use various_events::{Disconnect, NewConnection};

pub struct CraftFlow {
	connections: RwLock<Slab<ConnectionHandle>>,
	pub modules: Modules,
	pub reactor: Reactor<CraftFlow>,
}

impl CraftFlow {
	pub fn new() -> Self {
		Self {
			connections: RwLock::new(Slab::new()),
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

			let id = ConnectionHandle::add(&craftflow, stream);

			// Emit the new connection event
			if craftflow
				.reactor
				.event::<NewConnection>(&craftflow, id)
				.is_break()
			{
				// immediately disconnect the client
				craftflow.disconnect(id);
			}
		}
	}
	/// Accesses the connection handle of the given connection ID
	pub fn get<'a>(&'a self, conn_id: usize) -> MappedRwLockReadGuard<'a, ConnectionHandle> {
		RwLockReadGuard::map(self.connections.read().unwrap(), |inner| &inner[conn_id])
	}
	/// Disconnects the client with the given connection ID
	/// Panics if there is no client with the given connection ID
	pub fn disconnect(&self, conn_id: usize) {
		let mut connections = self.connections.write().unwrap();
		if !connections.contains(conn_id) {
			panic!("attempt to disconnect client with ID {conn_id} that does not exist");
		}

		// emit the disconnect event
		self.reactor.event::<Disconnect>(&self, conn_id);

		connections.remove(conn_id);
	}
	/// Accesses the connections slab
	/// There is no mutable access to the slab because the slab is not meant to be modified directly
	/// Use the `disconnect` method to disconnect a client
	pub fn connections(&self) -> RwLockReadGuard<Slab<ConnectionHandle>> {
		self.connections.read().unwrap()
	}
}
