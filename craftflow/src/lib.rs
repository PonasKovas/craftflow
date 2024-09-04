#![feature(mapped_lock_guards)]

mod connection_handle;
pub mod modules;
mod packet_events;
pub mod reactor;

use connection_handle::ConnectionHandle;
use modules::Modules;
use reactor::Reactor;
use slab::Slab;
use std::{
	ops::Deref,
	sync::{Arc, MappedRwLockReadGuard, RwLock, RwLockReadGuard},
};
use tokio::net::TcpListener;

pub struct CraftFlow {
	pub state: CFState,
	pub reactor: Reactor<CFState>,
}

/// The state of the CraftFlow server, accessible in the packet and base events
pub struct CFState {
	pub connections: Connections,
	pub modules: Modules,
}

/// All currently connected clients
pub struct Connections {
	inner: RwLock<Slab<ConnectionHandle>>,
}

impl CraftFlow {
	pub fn new() -> Self {
		Self {
			state: CFState {
				connections: Connections {
					inner: RwLock::new(Slab::new()),
				},
				modules: Modules::new(),
			},
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

			ConnectionHandle::add(&craftflow, stream);
		}
	}
}

impl Connections {
	/// Accesses the connection handle of the given connection ID
	pub fn get<'a>(&'a self, conn_id: usize) -> MappedRwLockReadGuard<'a, ConnectionHandle> {
		RwLockReadGuard::map(self.inner.read().unwrap(), |inner| &inner[conn_id])
	}
	/// Disconnects the client with the given connection ID
	/// Panics if there is no client with the given connection ID
	pub fn disconnect(&self, conn_id: usize) {
		self.inner.write().unwrap().remove(conn_id);
	}
}

impl Deref for Connections {
	type Target = RwLock<Slab<ConnectionHandle>>;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}
