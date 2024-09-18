use crate::{Login, VERIFY_TOKEN};
use craftflow::CraftFlow;
use craftflow_protocol::{
	datatypes::VarInt,
	protocol::{
		c2s::login::LoginStart,
		s2c::login::{EncryptionRequest, SetCompression},
	},
};
use rsa::traits::PublicKeyParts;
use std::ops::ControlFlow;

pub fn login_start(
	cf: &CraftFlow,
	(conn_id, request): (u64, LoginStart),
) -> ControlFlow<(), (u64, LoginStart)> {
	cf.modules
		.get::<Login>()
		.player_names_uuids
		.write()
		.unwrap()
		.insert(conn_id, (request.username.clone(), request.uuid));

	if let &Some(threshold) = &cf.modules.get::<Login>().compression_threshold {
		// Send the packet to enable compression
		cf.get(conn_id).send(SetCompression {
			threshold: VarInt(threshold as i32),
		});
	}

	if let Some(rsa_key) = &cf.modules.get::<Login>().rsa_key {
		// Send the packet to enable encryption
		cf.get(conn_id).send(EncryptionRequest {
			server_id: "".to_string(), // unused
			public_key: rsa_der::public_key_to_der(
				&rsa_key.n().to_bytes_be(),
				&rsa_key.e().to_bytes_be(),
			),
			verify_token: VERIFY_TOKEN.as_bytes().to_vec(),
			should_authenticate: false,
		});
	}

	ControlFlow::Continue((conn_id, request))
}
