use crate::{Login, VERIFY_TOKEN};
use craftflow::CraftFlow;
use craftflow_protocol::{
	c2s::login::{EncryptionBegin, encryption_begin::v759::Crypto},
	disabled_versions,
	s2c::login::{
		SuccessBuilder,
		success::{v5::SuccessV5, v735::SuccessV735, v759::SuccessV759, v766::SuccessV766},
	},
};
use rsa::Pkcs1v15Encrypt;
use std::ops::ControlFlow;
use tracing::error;

#[craftflow::callback(event: EncryptionBegin)]
pub async fn encryption_response(
	cf: &CraftFlow,
	&mut (conn_id, ref mut request): &mut (u64, EncryptionBegin),
) -> ControlFlow<()> {
	if let Some(rsa_key) = &cf.modules.get::<Login>().rsa_key {
		let shared_secret;
		let verify_token;

		match request {
			EncryptionBegin::V5(p) => {
				shared_secret = &p.shared_secret;
				verify_token = Some(&p.verify_token);
			}
			EncryptionBegin::V47(p) => {
				shared_secret = &p.shared_secret;
				verify_token = Some(&p.verify_token);
			}
			EncryptionBegin::V759(p) => {
				shared_secret = &p.shared_secret;
				match &p.crypto {
					Crypto::WithVerifyToken { verify_token: t } => {
						verify_token = Some(t);
					}
					Crypto::WithoutVerifyToken {
						salt: _,
						message_signature: _,
					} => {
						// TODO idk what to even do with this shit??
						verify_token = None;
					}
				}
			}
			disabled_versions!(c2s::login::EncryptionBegin) => unreachable!(),
		}

		match (
			rsa_key.decrypt(Pkcs1v15Encrypt, shared_secret),
			verify_token
				.map(|t| rsa_key.decrypt(Pkcs1v15Encrypt, &t))
				.transpose(),
		) {
			(Ok(decrypted_shared_secret), Ok(decrypted_verification_token)) => {
				// Check if the verification token is correct
				if let Some(token) = decrypted_verification_token {
					if &token != VERIFY_TOKEN.as_bytes() {
						error!("{} sent bad encryption response", cf.get(conn_id));
						cf.disconnect(conn_id).await;

						return ControlFlow::Break(());
					}
				}

				if decrypted_shared_secret.len() != 16 {
					error!("{} sent bad encryption response", cf.get(conn_id));
					cf.disconnect(conn_id).await;

					return ControlFlow::Break(());
				}

				let shared_secret: [u8; 16] = decrypted_shared_secret.try_into().unwrap();

				// Ok all good, set the shared secret for the connection and its done
				cf.get(conn_id).set_encryption(shared_secret);

				// get the player name and uuid that the client sent in the login start packet
				let info = cf
					.modules
					.get::<Login>()
					.player_names_uuids
					.read()
					.unwrap()
					.get(&conn_id)
					.cloned();
				let (username, uuid) = match info {
					Some(x) => x,
					None => {
						// Honestly I dont think this is possible, but just in case
						error!(
							"{} sent encryption response without sending login start",
							cf.get(conn_id)
						);
						cf.disconnect(conn_id).await;
						return ControlFlow::Break(());
					}
				};
				let uuid = uuid.unwrap_or(0);

				// And finish the login process
				cf.build_packet(conn_id, |b| match b {
					SuccessBuilder::V5(p) => p(SuccessV5 {
						uuid: format!(
							"{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
							(uuid >> (4 * 24)) & 0xffff_ffff,
							(uuid >> (4 * 20)) & 0xffff,
							(uuid >> (4 * 16)) & 0xffff,
							(uuid >> (4 * 12)) & 0xffff,
							uuid & 0xffff_ffff_ffff
						),
						username,
					}),
					SuccessBuilder::V735(p) => p(SuccessV735 { uuid, username }),
					SuccessBuilder::V759(p) => p(SuccessV759 {
						uuid,
						username,
						properties: Vec::new(),
					}),
					SuccessBuilder::V766(p) => p(SuccessV766 {
						uuid,
						username,
						properties: Vec::new(),
						strict_error_handling: false,
					}),
					disabled_versions!(s2c::login::SuccessBuilder) => unreachable!(),
				})
				.await;
			}
			_ => {
				// couldnt decrypt the shared secret or verify token
				error!("{} sent bad encryption response :(", cf.get(conn_id));
				cf.disconnect(conn_id).await;
			}
		}
	}

	ControlFlow::Continue(())
}
