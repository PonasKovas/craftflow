use crate::{
	connection::packet_reader::{Decryptor, PacketReader},
	packet_events::{trigger_c2s_abstract, trigger_c2s_concrete},
	CraftFlow,
};
use aes::cipher::KeyIvInit;
use anyhow::Context;
use craftflow_protocol_abstract::{
	AbC2S, AbPacketConstructor, AbPacketNew, ConstructorResult, State,
};
use craftflow_protocol_versions::{c2s, C2S};
use std::sync::{Arc, OnceLock, RwLock};
use tracing::error;

pub(super) async fn reader_task(
	craftflow: Arc<CraftFlow>,
	conn_id: u64,
	version: u32,
	mut reader: PacketReader,
	reader_state: Arc<RwLock<State>>,
	compression: Arc<OnceLock<usize>>,
	encryption_secret: Arc<OnceLock<[u8; 16]>>,
) -> anyhow::Result<()> {
	let mut constructors: Vec<
		Box<dyn AbPacketConstructor<'static, Direction = C2S, AbPacket = AbC2S> + Send + Sync>,
	> = Vec::new();

	let mut decryptor = None;

	loop {
		if decryptor.is_none() {
			// check if encryption secret received
			if let Some(secret) = encryption_secret.get() {
				decryptor = Some(Decryptor::new(secret.into(), secret.into()));
			}
		}

		reader
			.read_packet(&reader_state, version, &compression, &mut decryptor, |mut packet| {
    			// Handle some special packets which change the state of the connection
    			match packet {
    				C2S::Login(c2s::Login::LoginAcknowledged(_)) => {
    					*reader_state.write().unwrap() = State::Configuration;
    				}
    				C2S::Configuration(c2s::Configuration::FinishConfiguration(_)) => {
    					*reader_state.write().unwrap() = State::Play;
    				}
    				_ => {}
    			}

    			// trigger concrete packet event
    			// if trigger_c2s_concrete(false, &craftflow, conn_id, &mut packet).is_break() {
    			// 	return Ok(());
    			// }

    			// try to construct abstract packet
    			let mut abstr = 'abstr: {
    				// check all already started constructors
    				for i in 0..constructors.len() {
    					match constructors.get_mut(i).unwrap().next_packet(&packet)? {
    						ConstructorResult::Done(p) => {
                                constructors.remove(i);
                                break 'abstr p
                            },
    						ConstructorResult::Continue(()) => {
    							return Ok(());
    						}
    						ConstructorResult::Ignore => {}
    					}
    				}

    				// Otherwise try constructing a new one
    				match AbC2S::construct(&packet)? {
    					ConstructorResult::Done(p) => break 'abstr p,
    					ConstructorResult::Continue(c) => {
    						constructors.push(c.clone());
    						return Ok(());
    					}
    					ConstructorResult::Ignore => {
    						error!(
                                "Failed to construct abstract packet from concrete packet: {:?} (This is most likely a bug within craftflow)",
                                packet
                            );
    						return Ok(());
    					}
    				}
    			};

    			// trigger_c2s_abstract(false, &craftflow, conn_id, &mut abstr);
    			// trigger_c2s_abstract(true, &craftflow, conn_id, &mut abstr);
                // trigger_c2s_concrete(true, &craftflow, conn_id, &mut packet);

                Ok(())
			})
			.await
			.with_context(|| {
				format!(
					"reading concrete packet (state {:?})",
					reader_state.read().unwrap()
				)
			})?;
	}
}
