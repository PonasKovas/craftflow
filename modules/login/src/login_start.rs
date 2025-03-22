use crate::{Login, VERIFY_TOKEN};
use craftflow::CraftFlow;
use craftflow_protocol::{
	c2s::login::LoginStart,
	disabled_versions,
	s2c::login::{
		CompressBuilder, EncryptionBeginBuilder,
		compress::v47::CompressV47,
		encryption_begin::{
			v5::EncryptionBeginV5, v47::EncryptionBeginV47, v766::EncryptionBeginV766,
		},
	},
};
use rsa::traits::PublicKeyParts;
use std::ops::ControlFlow;

#[craftflow::callback(event: LoginStart)]
pub async fn login_start(
	cf: &CraftFlow,
	&mut (conn_id, ref mut request): &mut (u64, LoginStart),
) -> ControlFlow<()> {
	let username;
	let mut uuid = None;

	match request {
		disabled_versions!(c2s::login::LoginStart) => unreachable!(),
		LoginStart::V5(p) => {
			username = &p.username;
		}
		LoginStart::V759(p) => {
			username = &p.username;
		}
		LoginStart::V760(p) => {
			username = &p.username;
			uuid = p.player_uuid;
		}
		LoginStart::V761(p) => {
			username = &p.username;
			uuid = p.player_uuid;
		}
		LoginStart::V764(p) => {
			username = &p.username;
			uuid = Some(p.player_uuid);
		}
	}

	cf.modules
		.get::<Login>()
		.player_names_uuids
		.write()
		.unwrap()
		.insert(conn_id, (username.clone(), uuid));

	let version = cf.get(conn_id).protocol_version();

	if let &Some(threshold) = &cf.modules.get::<Login>().compression_threshold {
		// Send the packet to enable compression
		let packet = match CompressBuilder::new(version) {
			disabled_versions!(s2c::login::CompressBuilder) => unreachable!(),
			CompressBuilder::V47(p) => p(CompressV47 {
				threshold: threshold as i32,
			}),
		};
		cf.get(conn_id).send(packet).await;
	}

	if let Some(rsa_key) = &cf.modules.get::<Login>().rsa_key {
		// Send the packet to enable encryption
		let server_id = "".to_owned();
		let public_key =
			rsa_der::public_key_to_der(&rsa_key.n().to_bytes_be(), &rsa_key.e().to_bytes_be())
				.try_into()
				.expect("public key somehow too long?");
		let verify_token = VERIFY_TOKEN.as_bytes().try_into().unwrap();

		let packet = match EncryptionBeginBuilder::new(version) {
			disabled_versions!(s2c::login::EncryptionBeginBuilder) => unreachable!(),
			EncryptionBeginBuilder::V5(p) => p(EncryptionBeginV5 {
				server_id,
				public_key,
				verify_token,
			}),
			EncryptionBeginBuilder::V47(p) => p(EncryptionBeginV47 {
				server_id,
				public_key,
				verify_token,
			}),
			EncryptionBeginBuilder::V766(p) => p(EncryptionBeginV766 {
				server_id,
				public_key,
				verify_token,
				should_authenticate: true,
			}),
		};
		cf.get(conn_id).send(packet).await;
	}

	ControlFlow::Continue(())
}
