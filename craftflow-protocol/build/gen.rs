use super::{
	state_spec::StateSpec,
	util::{Direction, StateName},
	Info,
};
use crate::build::{spec_to_generator::spec_to_generator, version_bounds::BoundsMethods};
use feature::{Feature, FeatureCfgOptions};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::BTreeMap;

pub mod custom_format;
pub mod direction_generator;
pub mod enum_generator;
pub mod feature;
pub mod field;
pub mod fields_container;
pub mod generics;
pub mod packet_generator;
pub mod state_generator;
pub mod struct_generator;
pub mod version_dependent;

pub fn generate_code(
	info: &Info,
	c2s_states: BTreeMap<StateName, StateSpec>,
	s2c_states: BTreeMap<StateName, StateSpec>,
) -> TokenStream {
	let c2s_enum_variants = gen_state_enum_variants(Direction::C2S, &c2s_states);
	let s2c_enum_variants = gen_state_enum_variants(Direction::S2C, &s2c_states);

	let c2s_generator = spec_to_generator(Direction::C2S, c2s_states);
	let s2c_generator = spec_to_generator(Direction::S2C, s2c_states);

	let c2s_module = c2s_generator.gen(info);
	let s2c_module = s2c_generator.gen(info);

	let supported_versions = gen_supported_versions_list(info);

	quote! {
		#[doc = "All possible Client -> Server packets."]
		#[derive(Debug, Clone, PartialEq)]
		pub enum C2S<'a> {
			LegacyPing(crate::stable_packets::c2s::legacy::LegacyPing),
			Handshake(crate::stable_packets::c2s::handshake::Handshake<'a>),
			Status(crate::stable_packets::c2s::StatusPacket),
			#( #c2s_enum_variants )*
		}
		#[doc = "All possible Server -> Client packets."]
		#[derive(Debug, Clone, PartialEq)]
		pub enum S2C<'a> {
			LegacyPingResponse(crate::stable_packets::s2c::legacy::LegacyPingResponse),
			Status(crate::stable_packets::s2c::StatusPacket<'a>),
			#( #s2c_enum_variants )*
		}

		#[doc = "Contains Client -> Server packets."]
		pub mod c2s {
			#c2s_module
		}
		#[doc = "Contains Server -> Client packets."]
		pub mod s2c {
			#s2c_module
		}

		#[doc = "A list of all supported protocol versions with currently enabled features."]
		pub const SUPPORTED_PROTOCOL_VERSIONS: &[u32] = &[
			#( #supported_versions )*
		];
	}
}

fn gen_state_enum_variants(
	direction: Direction,
	state_spec: &BTreeMap<StateName, StateSpec>,
) -> Vec<TokenStream> {
	let mut variants = Vec::new();

	for (name, spec) in state_spec {
		let feature = spec.feature.as_ref().map(|f| {
			Feature { feature: f.clone() }.gen_cfg(FeatureCfgOptions {
				negative: false,
				with_doc_note: true,
			})
		});
		let variant_name = name.direction_enum_variant();
		let direction_module = direction.module();
		let state_enum_name = name.enum_name();

		variants.push(quote! {
			#feature
			#variant_name(#direction_module::#state_enum_name<'a>),
		});
	}

	variants
}

fn gen_supported_versions_list(info: &Info) -> Vec<TokenStream> {
	let all_supported = info.all_supported();

	all_supported
		.iter()
		.map(|&version| {
			// calculate a list of features that would block this version
			let blocking_features = info.features.iter().filter_map(|(feature_name, bounds)| {
				if bounds.contain(version) {
					None
				} else {
					Some(feature_name)
				}
			});

			quote! {
				#[cfg(not(any( #( feature = #blocking_features ),* )))]
				#version,
			}
		})
		.collect()
}
