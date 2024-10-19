use crate::{
	connection::packet_writer::PacketWriter,
	packet_events::{trigger_s2c_concrete_post, trigger_s2c_concrete_pre},
	CraftFlow,
};
use craftflow_protocol_versions::S2C;
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedReceiver;

/// The task that handles writing packets to the client.
pub(super) async fn writer_task(
	craftflow: Arc<CraftFlow>,
	conn_id: u64,
	mut writer: PacketWriter,
	mut packet_sender: UnboundedReceiver<S2C>,
) -> anyhow::Result<()> {
	loop {
		let mut packet = match packet_sender.recv().await {
			Some(p) => p,
			None => return Ok(()), // This means the connection has to be closed, as the handle was dropped
		};

		// trigger the packet event, and actually send it if it was not cancelled
		if trigger_s2c_concrete_pre(&craftflow, conn_id, &mut packet).is_continue() {
			writer.send(&packet).await?;

			let _ = trigger_s2c_concrete_post(&craftflow, conn_id, &mut packet);
		}
	}
}
