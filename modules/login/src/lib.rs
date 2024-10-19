#![doc(
	html_favicon_url = "https://github.com/PonasKovas/craftflow/blob/master/assets/icon.png?raw=true"
)]
#![doc(
	html_logo_url = "https://github.com/PonasKovas/craftflow/blob/master/assets/icon.png?raw=true"
)]

mod encryption_response;
mod login_start;
mod set_compression;

use craftflow::{packet_events::Post, various_events::Disconnect, CraftFlow};
use craftflow_protocol_abstract::c2s::{AbLoginEncryption, AbLoginStart};
use craftflow_protocol_versions::s2c::login::compress::v00765::CompressV00047;
use encryption_response::encryption_response;
use login_start::login_start;
use rsa::RsaPrivateKey;
use std::{collections::BTreeMap, ops::ControlFlow, sync::RwLock};

/// A module that handles the login phase of the minecraft protocol
/// This includes:
/// - Enabling encryption, if you want
/// - Enabling compression, if you want
pub struct Login {
	pub rsa_key: Option<RsaPrivateKey>,
	pub compression_threshold: Option<usize>,
	// The usernames and UUIDs that the client sends in the LoginStart packet
	pub player_names_uuids: RwLock<BTreeMap<u64, (String, Option<u128>)>>,
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

		craftflow
			.reactor
			.add_handler::<AbLoginStart, _>(login_start);
		craftflow
			.reactor
			.add_handler::<Post<CompressV00047>, _>(set_compression::set_compression);
		craftflow
			.reactor
			.add_handler::<AbLoginEncryption, _>(encryption_response);

		craftflow
			.reactor
			.add_handler::<Disconnect, _>(cleanup_player_names_uuids);
	}
}

fn cleanup_player_names_uuids(cf: &CraftFlow, conn_id: u64) -> ControlFlow<(), u64> {
	cf.modules
		.get::<Login>()
		.player_names_uuids
		.write()
		.unwrap()
		.remove(&conn_id);

	ControlFlow::Continue(conn_id)
}

impl Default for Login {
	fn default() -> Self {
		Self::new().enable_compression(256).enable_encryption(2048)
	}
}
