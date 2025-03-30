nbt!({
	"minecraft:banner_pattern": {},
	"minecraft:chat_type": {
		"minecraft:chat": {
			"chat": {
				"parameters": ["sender", "content"],
				"translation_key": "chat.type.text"
			},
			"narration": {
				"parameters": ["sender", "content"],
				"translation_key": "chat.type.text.narrate"
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
	"minecraft:painting_variant": {
		"minecraft:default": {
			"asset_id": "minecraft:alban",
			"height": 1i32,
			"width": 1i32
		}
	},
	"minecraft:trim_material": {},
	"minecraft:trim_pattern": {},
	"minecraft:wolf_variant": {
		"minecraft:default": {
			"angry_texture": "minecraft:entity/wolf/wolf_ashen_angry",
			"biomes": "minecraft:snowy_taiga",
			"tame_texture": "minecraft:entity/wolf/wolf_ashen_tame",
			"wild_texture": "minecraft:entity/wolf/wolf_ashen"
		}
	},
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
		},
		"minecraft:snowy_taiga": { // client checks for this biome specifically for some reason...
			"downfall": 0.4000000059604645f32,
			"effects": {
				"fog_color": 12638463i32,
				"mood_sound": {
					"block_search_extent": 8i32,
					"offset": 2.0f64,
					"sound": "minecraft:ambient.cave",
					"tick_delay": 6000i32
				},
				"sky_color": 8625919i32,
				"water_color": 4020182i32,
				"water_fog_color": 329011i32
			},
			"has_precipitation": 1i8,
			"temperature": -0.5f32
		}
	}
})