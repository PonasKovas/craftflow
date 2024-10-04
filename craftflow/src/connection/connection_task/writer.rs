use crate::{
	connection::packet_writer::PacketWriter,
	packet_events::{trigger_s2c_post, trigger_s2c_pre},
	packets::S2CPacket,
	CraftFlow,
};
use anyhow::Result;
use craftflow_protocol_abstract::AbPacketWrite;
use craftflow_protocol_versions::S2C;
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedReceiver;

/// The task that handles writing packets to the client.
pub(super) async fn writer_task(
	craftflow: Arc<CraftFlow>,
	conn_id: u64,
	mut writer: PacketWriter,
	mut packet_sender: UnboundedReceiver<S2CPacket>,
) -> anyhow::Result<()> {
	loop {
		let mut packet = match packet_sender.recv().await {
			Some(p) => p,
			None => return Ok(()), // This means the connection has to be closed, as the handle was dropped
		};

		// trigger the packet event, and actually send it if it was not cancelled
		if trigger_s2c_pre(&craftflow, conn_id, &mut packet).is_continue() {
			match packet.clone() {
				S2CPacket::Abstract(ab_packet) => {
					// Construct concrete packets from this abstract packet
					let concrete_packets = ab_packet.convert(writer.get_protocol_version())?;
					for packet in concrete_packets {
						write_concrete(&craftflow, conn_id, &mut writer, packet).await?;
					}
				}
				S2CPacket::Concrete(packet) => {
					write_concrete(&craftflow, conn_id, &mut writer, packet).await?;
				}
			}

			let _ = trigger_s2c_post(&craftflow, conn_id, &mut packet);
		}
	}
}

async fn write_concrete(
	craftflow: &CraftFlow,
	conn_id: u64,
	writer: &mut PacketWriter,
	packet: S2C,
) -> Result<()> {
	let mut packet = S2CPacket::Concrete(packet);
	if trigger_s2c_pre(&craftflow, conn_id, &mut packet).is_continue() {
		let concrete = packet.assume_concrete();
		writer.send(&concrete).await?;
		packet = S2CPacket::Concrete(concrete);
	}
	let _ = trigger_s2c_post(&craftflow, conn_id, &mut packet);
	Ok(())
}
