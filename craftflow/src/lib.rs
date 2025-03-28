#![doc(
	html_favicon_url = "https://github.com/PonasKovas/craftflow/blob/master/assets/icon.png?raw=true"
)]
#![doc(
	html_logo_url = "https://github.com/PonasKovas/craftflow/blob/master/assets/icon.png?raw=true"
)]
// used for Craftflow::get to get access to a connection through a lock
#![feature(mapped_lock_guards)]

pub use closureslop::{self, add_callback};
pub use craftflow_macros::{callback, init, reg};

pub mod connection;
pub mod modules;
pub mod packet_events;
pub mod various_events;

use closureslop::Reactor;
use connection::{ConnectionInterface, handle_new_conn};
use craftflow_protocol::{PacketBuilder, S2C};
use modules::Modules;
use std::{
	collections::HashMap,
	sync::{Arc, MappedRwLockReadGuard, RwLock, RwLockReadGuard},
};
use tokio::{net::TcpListener, spawn};
use tracing::error;
use various_events::{Disconnect, NewConnection};

pub struct CraftFlow {
	connections: RwLock<Connections>,
	pub modules: Modules,
	pub reactor: Reactor<CraftFlow>,
}

struct Connections {
	connections: HashMap<u64, Arc<ConnectionInterface>>,
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
			let (stream, socket_addr) = listener.accept().await?;

			// Emit the new connection event
			if craftflow
				.reactor
				.trigger::<NewConnection>(&craftflow, &mut socket_addr.ip())
				.await
				.is_break()
			{
				continue;
			}

			let craftflow_clone = Arc::clone(&craftflow);
			spawn(async move {
				if let Err(e) = handle_new_conn(craftflow_clone, stream, socket_addr).await {
					error!("handling new connection: {e:?}");
				}
			});
		}
	}
	/// Accesses the connection handle of the given connection ID
	pub fn get(&self, conn_id: u64) -> Arc<ConnectionInterface> {
		Arc::clone(&self.connections.read().unwrap().connections[&conn_id])
	}
	/// Convenience function for building and sending a packet to a connection
	pub async fn build_packet<B: PacketBuilder>(&self, conn_id: u64, f: impl FnOnce(B) -> B::Packet)
	where
		B::Packet: Into<S2C>,
	{
		let conn = self.get(conn_id);

		let builder = B::new(conn.protocol_version());
		let packet = f(builder);

		conn.send(packet).await;
	}
	/// Disconnects the client with the given connection ID
	/// No-op if the client is already disconnected, panic if the client ID was never connected
	pub async fn disconnect(&self, conn_id: u64) {
		if self.connections.read().unwrap().is_connected(conn_id) {
			// emit the disconnect event
			let _ = self
				.reactor
				.trigger::<Disconnect>(&self, &mut conn_id.clone())
				.await;

			self.connections
				.write()
				.unwrap()
				.connections
				.remove(&conn_id);
		}
	}
	/// Accesses the connections map
	/// There is no mutable access because it is not meant to be modified directly
	/// Use the `disconnect` method to disconnect a client
	pub fn connections<'a>(
		&'a self,
	) -> MappedRwLockReadGuard<'a, HashMap<u64, Arc<ConnectionInterface>>> {
		RwLockReadGuard::map(self.connections.read().unwrap(), |inner| &inner.connections)
	}
}

impl Connections {
	/// Checks if the given connection ID is connected
	///
	/// Panics if this ID was never connected
	fn is_connected(&self, conn_id: u64) -> bool {
		if self.next_conn_id <= conn_id {
			panic!("attempt to check client with ID {conn_id} that was never connected");
		}

		self.connections.contains_key(&conn_id)
	}
}
