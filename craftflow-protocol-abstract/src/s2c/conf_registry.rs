use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, WriteResult};
use anyhow::Result;
use crab_nbt::{Nbt, NbtCompound};
use craftflow_protocol_core::{common_structures::Text, datatypes::AnonymousNbt};
use craftflow_protocol_versions::{
	s2c::{
		configuration::{registry_data::v00765::RegistryDataV00764, RegistryData},
		Configuration,
	},
	IntoStateEnum, S2C,
};
use serde::{Deserialize, Serialize};
use std::{
	collections::HashMap,
	iter::{once, Once},
};

// Minecraft SLOP
// why tf is this even being sent
// half of it is not even being used, other half could just be handled completely on the server

#[derive(Debug, Clone, PartialEq)]
pub struct AbConfRegistry {
	/// Armor trim material registry
	pub trim_material: HashMap<String, TrimMaterialEntry>,
	/// Armor trim pattern registry
	pub trim_pattern: HashMap<String, TrimPatternEntry>,
	/// Biome registry
	pub biome: HashMap<String, BiomeEntry>,
	/// Chat type registry
	pub chat_type: HashMap<String, ChatTypeEntry>,
	/// Damage type registry
	pub damage_type: HashMap<String, DamageTypeEntry>,
	/// Dimension type registry
	pub dimension_type: HashMap<String, DimensionTypeEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrimMaterialEntry {
	pub asset_name: String,
	pub ingredient: String,
	pub item_model_index: f32,
	pub override_armor_materials: HashMap<String, String>,
	pub description: Text,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrimPatternEntry {
	pub asset_name: String,
	pub template_item: String,
	pub description: Text,
	pub decal: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BiomeEntry {
	pub has_precipitation: bool,
	pub temperature: f32,
	pub temperature_modifier: TeperatureModifier,
	pub downfall: f32,
	pub effects: Effects,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatTypeEntry {
	pub chat: Decoration,
	pub narration: Decoration,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DamageTypeEntry {
	pub message_id: String,
	pub scaling: DamageScaling,
	pub exhaustion: f32,
	pub effects: Option<DamageEffect>,
	pub death_message_type: Option<DeathMessageType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DimensionTypeEntry {
	pub fixed_time: Option<u64>,
	pub has_skylight: bool,
	pub has_ceiling: bool,
	pub ultrawarm: bool,
	pub natural: bool,
	pub coordinate_scale: f64,
	pub bed_works: bool,
	pub respawn_anchor_works: bool,
	pub min_y: i32,
	pub height: i32,
	pub logical_height: i32,
	pub infiniburn: String,
	pub effects: DimensionEffects,
	pub ambient_light: f32,
	pub piglin_safe: bool,
	pub has_raids: bool,
	pub monster_spawn_light_level: IntegerDistribution,
	pub monster_spawn_block_light_limit: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TeperatureModifier {
	None,
	Frozen,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Effects {
	pub fog_color: u32,
	pub water_color: u32,
	pub water_fog_color: u32,
	pub sky_color: u32,
	pub foliage_color: Option<u32>,
	pub grass_color: Option<u32>,
	pub grass_color_modifier: Option<GrassColorModifier>,
	pub particle: Option<Particle>,
	pub ambient_sound: Option<AmbientSound>,
	pub mood_sound: Option<MoodSound>,
	pub additions_sound: Option<AdditionsSound>,
	pub music: Option<Music>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GrassColorModifier {
	None,
	DarkForest,
	Swamp,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Particle {
	pub options: ParticleOptions,
	pub probability: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParticleOptions {
	// I think there are more options but the docs are not very clear
	#[serde(rename = "type")]
	pub particle_type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AmbientSound {
	Simple(String),
	WithOptions { sound: String, range: Option<f32> },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MoodSound {
	pub sound: String,
	pub tick_delay: u32,
	pub block_search_extent: u32,
	pub offset: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdditionsSound {
	pub sound: String,
	pub tick_chance: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Music {
	pub sound: String,
	pub min_delay: u32,
	pub max_delay: u32,
	pub replace_current_music: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Decoration {
	pub translation_key: String,
	pub parameters: Vec<DecorationParam>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DecorationParam {
	Sender,
	Target,
	Content,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DamageScaling {
	Never,
	WhenCausedByLivingNonPlayer,
	Always,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DamageEffect {
	Hurt,
	Thorns,
	Drowning,
	Burning,
	Poking,
	Freezing,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeathMessageType {
	Default,
	FallVariants,
	IntentionalGameDesign,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DimensionEffects {
	Overworld,
	TheNether,
	TheEnd,
}

// This is not really accurate, but i dont see the point to make it accurate since this is literally
// useless information, the server sends it for no reason, the client doesnt use it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IntegerDistribution {
	Integer(i32),
	Distribution {
		distribution_type: String,
		value: DistributionRange,
	},
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DistributionRange {
	pub min_inclusive: i32,
	pub max_inclusive: i32,
}

impl AbPacketWrite for AbConfRegistry {
	type Direction = S2C;
	type Iter = Once<Self::Direction>;

	fn convert(self, protocol_version: u32) -> Result<WriteResult<Self::Iter>> {
		let mut nbt = NbtCompound::new();

		// todo

		let pkt = match protocol_version {
			764.. => RegistryDataV00764 {
				codec: AnonymousNbt {
					inner: Nbt::new(String::new(), nbt),
				},
			}
			.into_state_enum(),
			_ => return Ok(WriteResult::Unsupported),
		};

		Ok(WriteResult::Success(once(pkt)))
	}
}

impl AbPacketNew for AbConfRegistry {
	type Direction = S2C;
	type Constructor = NoConstructor<Self, S2C>;

	fn construct(
		packet: Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor, Self::Direction>> {
		Ok(match packet {
			S2C::Configuration(Configuration::RegistryData(RegistryData::V00764(pkt))) => {
				ConstructorResult::Done(Self {
					trim_material: todo!(),
					trim_pattern: todo!(),
					biome: todo!(),
					chat_type: todo!(),
					damage_type: todo!(),
					dimension_type: todo!(),
				})
			}
			_ => ConstructorResult::Ignore(packet),
		})
	}
}
