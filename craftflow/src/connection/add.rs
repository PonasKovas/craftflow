use super::{
	connection_task::connection_task, packet_reader::PacketReader, packet_writer::PacketWriter,
	ConnectionInterface,
};
use crate::CraftFlow;
use craftflow_protocol_abstract::State;
use futures::FutureExt;
use std::{
	panic::AssertUnwindSafe,
	sync::{Arc, OnceLock, RwLock},
};
use tokio::{net::TcpStream, spawn, sync::mpsc};
use tracing::error;

/// Spawns the reading and writing tasks for a client connection.
/// And adds the connection interface to the craftflow instance
/// returns the ID of the connection
pub(crate) fn new_conn_interface(craftflow: &Arc<CraftFlow>, stream: TcpStream) -> u64 {
	let peer_ip = stream.peer_addr().unwrap().ip();

	let (concrete_packet_sender_in, concrete_packet_sender_out) = mpsc::unbounded_channel();
	let (abstract_packet_sender_in, abstract_packet_sender_out) = mpsc::unbounded_channel();

	let reader_state = Arc::new(RwLock::new(State::Handshake));
	let writer_state = Arc::new(RwLock::new(State::Handshake));
	let compression = Arc::new(OnceLock::new());
	let encryption_secret = Arc::new(OnceLock::new());
	let protocol_version = Arc::new(OnceLock::new());

	// Insert into the connections slab
	let id = {
		let mut lock = craftflow.connections.write().unwrap();

		let id = lock.next_conn_id;
		lock.next_conn_id += 1;

		lock.connections.insert(
			id,
			ConnectionInterface {
				id,
				ip: peer_ip,
				concrete_packet_sender: concrete_packet_sender_in,
				abstract_packet_sender: abstract_packet_sender_in,
				encryption_secret: Arc::clone(&encryption_secret),
				compression: Arc::clone(&compression),
				writer_state: Arc::clone(&writer_state),
				protocol_version: Arc::clone(&protocol_version),
			},
		);

		id
	};

	let (reader, writer) = stream.into_split();

	let packet_reader = PacketReader::new(reader);
	let packet_writer = PacketWriter::new(writer);

	// Spawn a task for handling this connection
	let craftflow = Arc::clone(&craftflow);
	spawn(async move {
		let r = AssertUnwindSafe(connection_task(
			Arc::clone(&craftflow),
			id,
			packet_reader,
			packet_writer,
			concrete_packet_sender_out,
			abstract_packet_sender_out,
			reader_state,
			writer_state,
			protocol_version,
			compression,
			encryption_secret,
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
		craftflow.disconnect(id).await;
	});

	id
}
