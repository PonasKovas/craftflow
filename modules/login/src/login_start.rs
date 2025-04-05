use crate::{Login, VERIFY_TOKEN};
use craftflow::{ConnId, CraftFlow, packet_events::Packet};
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
use std::{ops::ControlFlow, sync::Arc};

#[craftflow::callback(event: Packet<LoginStart>)]
pub async fn login_start(
	cf: &Arc<CraftFlow>,
	&mut (conn_id, ref mut request): &mut (ConnId, LoginStart),
) -> ControlFlow<()> {
	let username;
	let mut uuid = None;

	match request {
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
		disabled_versions!(c2s::login::LoginStart) => unreachable!(),
	}

	cf.modules
		.get::<Login>()
		.player_names_uuids
		.write()
		.unwrap()
		.insert(conn_id, (username.clone(), uuid));

	if let &Some(threshold) = &cf.modules.get::<Login>().compression_threshold {
		// Send the packet to enable compression
		cf.build_packet(conn_id, |b| match b {
			disabled_versions!(s2c::login::CompressBuilder) => unreachable!(),
			CompressBuilder::V47(p) => p(CompressV47 {
				threshold: threshold as i32,
			}),
		})
		.await;
	}

	if let Some(rsa_key) = &cf.modules.get::<Login>().rsa_key {
		// Send the packet to enable encryption
		let server_id = "".to_owned();
		let public_key =
			rsa_der::public_key_to_der(&rsa_key.n().to_bytes_be(), &rsa_key.e().to_bytes_be());
		let verify_token = VERIFY_TOKEN.as_bytes().to_owned();

		cf.build_packet(conn_id, |b| match b {
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
			disabled_versions!(s2c::login::EncryptionBeginBuilder) => unreachable!(),
		})
		.await;
	}

	ControlFlow::Continue(())
}
