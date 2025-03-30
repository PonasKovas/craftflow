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

use craftflow::{CraftFlow, various_events::Disconnect};
use craftflow_protocol::craftflow_nbt::{NbtValue, nbt};
use rsa::RsaPrivateKey;
use std::{collections::BTreeMap, ops::ControlFlow, sync::RwLock};

craftflow::init!(ctx: CraftFlow);

/// A module that handles the login phase of the minecraft protocol
/// This includes:
/// - Enabling encryption, if you want
/// - Enabling compression, if you want
pub struct Login {
	pub rsa_key: Option<RsaPrivateKey>,
	pub compression_threshold: Option<usize>,
	// The usernames and UUIDs that the client sends in the LoginStart packet
	pub player_names_uuids: RwLock<BTreeMap<u64, (String, Option<u128>)>>,
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
			registry_data: nbt!({
			"minecraft:banner_pattern": {},
			"minecraft:chat_type": {
				"minecraft:chat": {
					"minecraft:chat": {
						"chat": {
							"parameters": ["sender", "content"],
							"translation_key": "chat.type.text"
						},
						"narration": {
							"parameters": ["sender", "content"],
							"translation_key": "chat.type.text.narrate"
						}
					}
				},
			},
			"minecraft:damage_type": {
				"minecraft:generic": {
					"exhaustion": 0f32,
					"message_id": "generic",
					"scaling": "when_caused_by_living_non_player"
				}
			},
			"minecraft:dimension_type": {
				"minecraft:default": {
					"ambient_light": 0f32,
					"bed_works": 1i8,
					"coordinate_scale": 1f64,
					"effects": "minecraft:overworld",
					"has_ceiling": 0i8,
					"has_raids": 1i8,
					"has_skylight": 1i8,
					"height": 384i32,
					"infiniburn": "#minecraft:infiniburn_overworld",
					"logical_height": 384i32,
					"min_y": -64i32,
					"monster_spawn_block_light_limit": 0i32,
					"monster_spawn_light_level": {
						"max_inclusive": 7i32,
						"min_inclusive": 0i32,
						"type": "minecraft:uniform"
					},
					"natural": 1i8,
					"piglin_safe": 0i8,
					"respawn_anchor_works": 0i8,
					"ultrawarm": 0i8
				}
			},
			"minecraft:painting_variant": {},
			"minecraft:trim_material": {},
			"minecraft:trim_pattern": {},
			"minecraft:wolf_variant": {},
			"minecraft:worldgen/biome": {
				"minecraft:default": {
					"downfall": 0.4000000059604645f32,
					"effects": {
						"fog_color": 12638463i32,
						"mood_sound": {
							"block_search_extent": 8i32,
							"offset": 2.0f64,
							"sound": "minecraft:ambient.cave",
							"tick_delay": 6000i32
						},
						"sky_color": 7907327i32,
						"water_color": 4159204i32,
						"water_fog_color": 329011i32
					},
					"has_precipitation": 1i8,
					"temperature": 0.800000011920929f32
				}
			}
							  }),
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
async fn cleanup_player_names_uuids(cf: &CraftFlow, conn_id: &mut u64) -> ControlFlow<()> {
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
