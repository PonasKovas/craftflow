use crate::{Login, VERIFY_TOKEN};
use craftflow::CraftFlow;
use craftflow_protocol_abstract::{
	c2s::AbLoginStart,
	s2c::{AbLoginCompress, AbLoginEncryptionBegin},
};
use rsa::traits::PublicKeyParts;
use std::ops::ControlFlow;

pub fn login_start<'a>(
	cf: &CraftFlow,
	(conn_id, request): (u64, &'a mut AbLoginStart),
) -> ControlFlow<(), (u64, &'a mut AbLoginStart)> {
	cf.modules
		.get::<Login>()
		.player_names_uuids
		.write()
		.unwrap()
		.insert(conn_id, (request.username.clone(), request.uuid));

	if let &Some(threshold) = &cf.modules.get::<Login>().compression_threshold {
		// Send the packet to enable compression
		cf.get(conn_id).send(AbLoginCompress {
			threshold: threshold as i32,
		});
	}

	if let Some(rsa_key) = &cf.modules.get::<Login>().rsa_key {
		// Send the packet to enable encryption
		cf.get(conn_id).send(AbLoginEncryptionBegin {
			server_id: "".to_string(), // unused
			public_key: rsa_der::public_key_to_der(
				&rsa_key.n().to_bytes_be(),
				&rsa_key.e().to_bytes_be(),
			),
			verify_token: VERIFY_TOKEN.as_bytes().to_vec(),
		});
	}

	ControlFlow::Continue((conn_id, request))
}
