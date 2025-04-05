#![doc(
	html_favicon_url = "https://github.com/PonasKovas/craftflow/blob/master/assets/icon.png?raw=true"
)]
#![doc(
	html_logo_url = "https://github.com/PonasKovas/craftflow/blob/master/assets/icon.png?raw=true"
)]

mod configuration;
mod encryption_response;
mod login_start;
mod set_compression;

use craftflow::{ConnId, CraftFlow, various_events::Disconnect};
use craftflow_protocol::craftflow_nbt::{NbtValue, nbt};
use rsa::RsaPrivateKey;
use std::{
	collections::BTreeMap,
	ops::ControlFlow,
	sync::{Arc, RwLock},
};

craftflow::init!();

/// A module that handles the login phase of the minecraft protocol
/// This includes:
/// - Enabling encryption, if you want
/// - Enabling compression, if you want
pub struct Login {
	pub rsa_key: Option<RsaPrivateKey>,
	pub compression_threshold: Option<usize>,
	// The usernames and UUIDs that the client sends in the LoginStart packet
	pub player_names_uuids: RwLock<BTreeMap<ConnId, (String, Option<u128>)>>,
	registry_data: NbtValue,
}

const VERIFY_TOKEN: &str = "craftflow easter egg! 🐇🐰 :D";

impl Login {
	/// Creates a new Login module instance with:
	/// - No encryption
	/// - No compression
	pub fn new() -> Self {
		Self {
			rsa_key: None,
			compression_threshold: None,
			player_names_uuids: RwLock::new(BTreeMap::new()),
			registry_data: include!("default_registry.rs"),
		}
	}
	/// Enables encryption with an RSA key of the given bit size
	/// Recommended bit size is 2048.
	pub fn enable_encryption(mut self, bit_size: usize) -> Self {
		let mut thread_rng = rand::thread_rng();
		let rsa_key = RsaPrivateKey::new(&mut thread_rng, bit_size).unwrap();

		self.rsa_key = Some(rsa_key);

		self
	}
	/// Disables encryption
	pub fn disable_encryption(mut self) -> Self {
		self.rsa_key = None;
		self
	}
	/// Enables compression with the given threshold
	/// Recommended threshold is 256.
	pub fn enable_compression(mut self, threshold: usize) -> Self {
		self.compression_threshold = Some(threshold);
		self
	}
	/// Disables compression
	pub fn disable_compression(mut self) -> Self {
		self.compression_threshold = None;
		self
	}

	/// Adds the module to a CraftFlow instance.
	pub fn register(self, craftflow: &mut CraftFlow) {
		craftflow.modules.register(self);

		craftflow::reg!(to: &mut craftflow.reactor);
	}
}

#[craftflow::callback(event: Disconnect)]
async fn cleanup_player_names_uuids(cf: &Arc<CraftFlow>, conn_id: &mut ConnId) -> ControlFlow<()> {
	cf.modules
		.get::<Login>()
		.player_names_uuids
		.write()
		.unwrap()
		.remove(conn_id);

	ControlFlow::Continue(())
}

impl Default for Login {
	fn default() -> Self {
		Self::new().enable_compression(256).enable_encryption(2048)
	}
}
