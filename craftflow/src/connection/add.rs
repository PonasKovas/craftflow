use super::{
	connection_task, packet_reader::PacketReader, packet_writer::PacketWriter, ConnState,
	ConnectionHandle,
};
use crate::CraftFlow;
use futures::FutureExt;
use std::{
	io::Cursor,
	panic::AssertUnwindSafe,
	sync::{Arc, OnceLock, RwLock},
};
use tokio::{net::TcpStream, spawn, sync::mpsc};
use tracing::error;

impl ConnectionHandle {
	/// Spawns the reading and writing tasks for a client connection.
	/// And adds the connection handle to the craftflow instance
	/// returns the ID of the connection
	pub(crate) fn add(craftflow: &Arc<CraftFlow>, stream: TcpStream) -> u64 {
		let peer_ip = stream.peer_addr().unwrap().ip();

		let (packet_sender_in, packet_sender_out) = mpsc::unbounded_channel();

		let state = Arc::new(RwLock::new(ConnState::Handshake));
		let compression = Arc::new(OnceLock::new());
		let encryption_secret = Arc::new(OnceLock::new());

		let (reader, writer) = stream.into_split();

		let protocol_version = Arc::new(OnceLock::new());

		let packet_reader = PacketReader {
			stream: reader,
			buffer: Vec::with_capacity(1024 * 1024),
			decompression_buffer: Vec::with_capacity(1024 * 1024),
			state: Arc::clone(&state),
			encryption_secret: Arc::clone(&encryption_secret),
			decryptor: None,
			compression: Arc::clone(&compression),
			protocol_version: Arc::clone(&protocol_version),
		};
		let packet_writer = PacketWriter {
			stream: writer,
			buffer: Cursor::new(Vec::with_capacity(1024 * 1024)),
			state: Arc::clone(&state),
			encryption_secret: Arc::clone(&encryption_secret),
			encryptor: None,
			compression: Arc::clone(&compression),
			protocol_version: Arc::clone(&protocol_version),
		};

		let protocol_version_clone = Arc::clone(&protocol_version);
		let state_clone = Arc::clone(&state);

		// Insert into the connections slab
		let conn_id = {
			let mut lock = craftflow.connections.write().unwrap();
			let id = lock.next_conn_id;
			lock.connections.insert(
				id,
				Self {
					craftflow: Arc::clone(&craftflow),
					id,
					ip: peer_ip,
					packet_sender: RwLock::new(packet_sender_in),
					encryption_secret,
					compression,
					state,
					protocol_version,
				},
			);

			lock.next_conn_id += 1;
			id
		};

		let craftflow = Arc::clone(craftflow);
		spawn(async move {
			let r = AssertUnwindSafe(connection_task(
				Arc::clone(&craftflow),
				conn_id,
				packet_reader,
				packet_writer,
				packet_sender_out,
				protocol_version_clone,
				state_clone,
			))
			.catch_unwind() // generally this shouldnt panic, but if it does, we still want to remove the connection
			.await;

			match r {
				Ok(Ok(_)) => {} // ended peacefully 😊
				Ok(Err(e)) => {
					error!("{e:?}");
				}
				Err(_) => {} // panicked... wow.. cringe
			}

			// remove the connection from the list
			craftflow.disconnect(conn_id);
		});

		conn_id
	}
}
