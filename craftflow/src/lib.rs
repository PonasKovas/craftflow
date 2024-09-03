mod connection_handle;
mod listener;
pub mod modules;
mod packet_events;
pub mod reactor;

use connection_handle::ConnectionHandle;
use listener::listener_task;
use modules::Modules;
use packet_events::trigger_packet_event;
use reactor::Reactor;
use slab::Slab;
use tokio::{
	net::TcpListener,
	spawn,
	sync::mpsc::{self},
};
use tracing::info;

pub struct CraftFlow {
	pub state: CFState,
	pub reactor: Reactor<CFState>,
}

/// The state of the CraftFlow server, accessible in the packet and base events
pub struct CFState {
	pub connections: Slab<ConnectionHandle>,
	pub modules: Modules,
}

impl CraftFlow {
	pub fn new() -> Self {
		Self {
			state: CFState {
				connections: Slab::new(),
				modules: Modules::new(),
			},
			reactor: Reactor::new(),
		}
	}

	pub async fn run(mut self) -> anyhow::Result<()> {
		let listener = TcpListener::bind("0.0.0.0:25565").await?;
		let (new_conn_sender, mut new_conn_recv) = mpsc::channel(32);

		// spawn a task for accepting new connections
		spawn(listener_task(listener, new_conn_sender));

		// main loop
		loop {
			// add new connections
			for _ in 0..new_conn_recv.len() {
				let stream = new_conn_recv.try_recv().unwrap();
				let id = self.state.connections.insert(ConnectionHandle::new(stream));
				info!("new connection: {}", id);
			}

			// handle packets from all connections
			let ids: Vec<usize> = self.state.connections.iter().map(|(id, _)| id).collect();
			for conn_id in ids {
				'packets: loop {
					let conn = match self.state.connections.get_mut(conn_id) {
						Some(conn) => conn,
						None => continue, // connection might have been removed by some event handler
					};

					let packet = match conn.packet_receiver.try_recv() {
						Ok(packet) => packet,
						Err(_) => break 'packets,
					};

					trigger_packet_event(&mut self.reactor, &mut self.state, conn_id, packet);
				}
			}

			// remove disconnected connections
			self.state.connections.retain(|id, conn| {
				if conn.packet_receiver.is_closed() || conn.packet_sender.is_closed() {
					info!("connection {} disconnected", id);
					false
				} else {
					true
				}
			});
		}
	}
}
