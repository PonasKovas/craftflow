use crate::{Login, VERIFY_TOKEN};
use craftflow::CraftFlow;
use craftflow_protocol_abstract::{c2s::AbLoginEncryption, s2c::AbLoginSuccess};
use rsa::Pkcs1v15Encrypt;
use std::ops::ControlFlow;
use tracing::error;

pub fn encryption_response<'a>(
	cf: &CraftFlow,
	(conn_id, request): (u64, &'a mut AbLoginEncryption),
) -> ControlFlow<(), (u64, &'a mut AbLoginEncryption)> {
	if let Some(rsa_key) = &cf.modules.get::<Login>().rsa_key {
		match (
			rsa_key.decrypt(Pkcs1v15Encrypt, &request.shared_secret),
			request
				.verify_token
				.as_ref()
				.map(|t| rsa_key.decrypt(Pkcs1v15Encrypt, &t))
				.transpose(),
		) {
			(Ok(decrypted_shared_secret), Ok(decrypted_verification_token)) => {
				// Check if the verification token is correct
				if let Some(token) = decrypted_verification_token {
					if &token != VERIFY_TOKEN.as_bytes() {
						error!("{} sent bad encryption response", cf.get(conn_id));
						cf.disconnect(conn_id);

						return ControlFlow::Break(());
					}
				}

				if decrypted_shared_secret.len() != 16 {
					error!("{} sent bad encryption response", cf.get(conn_id));
					cf.disconnect(conn_id);

					return ControlFlow::Break(());
				}

				let shared_secret: [u8; 16] = decrypted_shared_secret.try_into().unwrap();

				// Ok all good, set the shared secret for the connection and its done
				cf.get(conn_id).set_encryption(shared_secret);

				// get the player name and uuid that the client sent in the login start packet
				let (username, uuid) = match cf
					.modules
					.get::<Login>()
					.player_names_uuids
					.read()
					.unwrap()
					.get(&conn_id)
				{
					Some((name, uuid)) => (name.clone(), *uuid),
					None => {
						// Honestly I dont think this is possible, but just in case
						error!(
							"{} sent encryption response without sending login start",
							cf.get(conn_id)
						);
						cf.disconnect(conn_id);
						return ControlFlow::Break(());
					}
				};

				// And finish the login process
				cf.get(conn_id).send(AbLoginSuccess {
					uuid: uuid.unwrap_or(0),
					username,
					properties: Vec::new(),
					strict_error_handling: false,
				});
			}
			_ => {
				// couldnt decrypt the shared secret or verify token
				error!("{} sent bad encryption response", cf.get(conn_id));
				cf.disconnect(conn_id);
			}
		}
	}

	ControlFlow::Continue((conn_id, request))
}
