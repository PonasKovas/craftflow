use crate::{Login, VERIFY_TOKEN};
use craftflow::{packet_events::C2SAbLoginStartEvent, CraftFlow};
use craftflow_protocol_abstract::{
	c2s::AbLoginStart,
	s2c::{AbLoginCompress, AbLoginEncryptionBegin},
};
use rsa::traits::PublicKeyParts;
use std::ops::ControlFlow;

#[craftflow::callback(event: C2SAbLoginStartEvent)]
pub async fn login_start(
	cf: &CraftFlow,
	&mut (conn_id, ref mut request): &mut (u64, AbLoginStart<'_>),
) -> ControlFlow<()> {
	cf.modules
		.get::<Login>()
		.player_names_uuids
		.write()
		.unwrap()
		.insert(conn_id, (request.username.to_string(), request.uuid));

	if let &Some(threshold) = &cf.modules.get::<Login>().compression_threshold {
		// Send the packet to enable compression
		cf.get(conn_id)
			.send(AbLoginCompress {
				threshold: threshold as i32,
			})
			.await;
	}

	if let Some(rsa_key) = &cf.modules.get::<Login>().rsa_key {
		// Send the packet to enable encryption
		cf.get(conn_id)
			.send(AbLoginEncryptionBegin {
				server_id: "".into(), // unused
				public_key: rsa_der::public_key_to_der(
					&rsa_key.n().to_bytes_be(),
					&rsa_key.e().to_bytes_be(),
				)
				.into(),
				verify_token: VERIFY_TOKEN.as_bytes().into(),
				should_authenticate: true,
			})
			.await;
	}

	ControlFlow::Continue(())
}
