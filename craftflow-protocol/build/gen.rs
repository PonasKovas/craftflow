use super::{
	state_spec::StateSpec,
	util::{to_pascal_case, AsIdent, Direction, StateName},
	Info,
};
use crate::build::spec_to_generator::spec_to_generator;
use feature::{Feature, FeatureCfgOptions};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::collections::BTreeMap;

pub mod custom_format;
pub mod direction_generator;
pub mod enum_generator;
pub mod feature;
pub mod field;
pub mod fields_container;
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

	quote! {
		#[doc = "All possible Client -> Server packets."]
		pub enum C2S {
			LegacyPing,
			#( #c2s_enum_variants )*
		}
		#[doc = "All possible Server -> Client packets."]
		pub enum S2C {
			LegacyPingResponse(crate::legacy::LegacyPingResponse),
			#( #s2c_enum_variants )*
		}

		#[doc = "Contains Client -> Server packets."]
		pub mod c2s {
			#[allow(unused_imports)]
			use crate::datatypes::*;
			#[allow(unused_imports)]
			use std::borrow::Borrow;

			#c2s_module
		}
		#[doc = "Contains Server -> Client packets."]
		pub mod s2c {
			#[allow(unused_imports)]
			use crate::datatypes::*;
			#[allow(unused_imports)]
			use std::borrow::Borrow;

			#s2c_module
		}
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
			#variant_name(#direction_module::#state_enum_name),
		});
	}

	variants
}
