#![doc(
	html_favicon_url = "https://github.com/PonasKovas/craftflow/blob/master/assets/icon.png?raw=true"
)]
#![doc(
	html_logo_url = "https://github.com/PonasKovas/craftflow/blob/master/assets/icon.png?raw=true"
)]
// used for Craftflow::get to get access to a connection through a lock
#![feature(mapped_lock_guards)]

use anyhow::bail;
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
	fmt::Display,
	ops::ControlFlow,
	sync::{Arc, MappedRwLockReadGuard, RwLock, RwLockReadGuard},
};
use tokio::{net::TcpListener, spawn};
use tracing::{error, info, trace};
use various_events::{Disconnect, Init, NewConnection};

pub struct CraftFlow {
	connections: RwLock<Connections>,
	pub modules: Modules,
	pub reactor: Reactor<Arc<CraftFlow>>,
}

struct Connections {
	connections: HashMap<ConnId, Arc<ConnectionInterface>>,
	next_conn_id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
pub struct ConnId(u64);

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

		if let ControlFlow::Break(msg) =
			craftflow.reactor.trigger::<Init>(&craftflow, &mut ()).await
		{
			bail!("CraftFlow could not initialize: {msg}");
		}

		info!("Craftflow started.");

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
	pub fn get(&self, conn_id: ConnId) -> Arc<ConnectionInterface> {
		Arc::clone(&self.connections.read().unwrap().connections[&conn_id])
	}
	/// Convenience function for building and sending a packet to a connection
	///
	/// WARNING: automatically checks if the client version has that packet. If not - does nothing silently.
	pub async fn build_packet<B: PacketBuilder>(
		&self,
		conn_id: ConnId,
		f: impl FnOnce(B) -> B::Packet,
	) where
		B::Packet: Into<S2C>,
	{
		let conn = self.get(conn_id);
		let version = conn.protocol_version();

		if !B::VERSIONS.contains(&version) {
			trace!(
				"Not building {:?} for conn {conn_id}, because not available in protocol version {version}.",
				std::any::type_name::<B>()
			);
			return;
		}

		let builder = B::new(version);
		let packet = f(builder);

		conn.send(packet).await;
	}
	/// Disconnects the client with the given connection ID
	/// No-op if the client is already disconnected, panic if the client ID was never connected
	pub async fn disconnect(self: &Arc<Self>, conn_id: ConnId) {
		if self.connections.read().unwrap().is_connected(conn_id) {
			// emit the disconnect event
			let _ = self
				.reactor
				.trigger::<Disconnect>(self, &mut { conn_id })
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
	) -> MappedRwLockReadGuard<'a, HashMap<ConnId, Arc<ConnectionInterface>>> {
		RwLockReadGuard::map(self.connections.read().unwrap(), |inner| &inner.connections)
	}
}

impl Connections {
	/// Checks if the given connection ID is connected
	///
	/// Panics if this ID was never connected
	fn is_connected(&self, conn_id: ConnId) -> bool {
		if self.next_conn_id <= conn_id.0 {
			panic!("attempt to check client with ID {conn_id} that was never connected");
		}

		self.connections.contains_key(&conn_id)
	}
}

impl Default for CraftFlow {
	fn default() -> Self {
		todo!()
	}
}

impl Display for ConnId {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "c{}", self.0)
	}
}
