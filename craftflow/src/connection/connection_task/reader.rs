use crate::{
	connection::packet_reader::PacketReader,
	packet_events::{trigger_c2s_abstract, trigger_c2s_concrete},
	CraftFlow,
};
use craftflow_protocol_abstract::{AbC2S, AbPacketConstructor, AbPacketNew, ConstructorResult};
use craftflow_protocol_core::Context;
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
		let mut packet = reader.read_packet().await.with_context(|| {
			format!(
				"reading concrete packet (state {:?})",
				reader.state.read().unwrap()
			)
		})?;

		// trigger concrete packet event
		if trigger_c2s_concrete(&craftflow, conn_id, &mut packet).is_break() {
			continue;
		}

		// try to construct abstract packet
		let mut abstr = 'abstr: {
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

		trigger_c2s_abstract(&craftflow, conn_id, &mut abstr);
	}
}
