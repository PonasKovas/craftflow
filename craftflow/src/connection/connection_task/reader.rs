use crate::{
	connection::packet_reader::PacketReader, packet_events::trigger_c2s, packets::C2SPacket,
	CraftFlow,
};
use craftflow_protocol_abstract::{AbC2S, AbPacketConstructor, AbPacketNew, ConstructorResult};
use craftflow_protocol_versions::C2S;
use std::sync::Arc;
use tracing::error;

pub(super) async fn reader_task(
	craftflow: Arc<CraftFlow>,
	conn_id: u64,
	mut reader: PacketReader,
) -> anyhow::Result<()> {
	let mut constructors: Vec<
		Box<dyn AbPacketConstructor<Direction = C2S, AbPacket = AbC2S> + Send + Sync>,
	> = Vec::new();

	'read_packet: loop {
		let packet = reader.read_packet().await?;

		let mut packet = C2SPacket::Concrete(packet);
		// trigger concrete packet event
		if trigger_c2s(&craftflow, conn_id, &mut packet).is_break() {
			continue;
		}

		// try to construct abstract packet
		let abstr = 'abstr: {
			let mut packet = packet.assume_concrete();

			// check all already started constructors
			for i in (0..constructors.len()).rev() {
				let constr = constructors.remove(i);

				match AbPacketConstructor::next_packet(constr, packet)? {
					ConstructorResult::Done(p) => break 'abstr p,
					ConstructorResult::Continue(constr) => {
						constructors.insert(i, constr);
						continue 'read_packet;
					}
					ConstructorResult::Ignore((constr, p)) => {
						constructors.insert(i, constr);
						packet = p;
					}
				}
			}

			// Otherwise try constructing a new one
			match AbC2S::construct(packet)? {
				ConstructorResult::Done(p) => break 'abstr p,
				ConstructorResult::Continue(c) => {
					constructors.push(c);
					continue 'read_packet;
				}
				ConstructorResult::Ignore(p) => {
					error!("Failed to construct abstract packet from concrete packet: {:?} (This is most likely a bug within craftflow)", p);
					continue 'read_packet;
				}
			}
		};

		trigger_c2s(&craftflow, conn_id, &mut C2SPacket::Abstract(abstr));
	}
}
