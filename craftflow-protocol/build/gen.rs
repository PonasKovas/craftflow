use super::{state_spec::StateSpec, Info};
use crate::build::{to_pascal_case, AsIdent};
use feature_cfg::gen_feature_cfg;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::BTreeMap;

pub mod feature_cfg;
pub mod state_module;

pub fn generate_code(
	info: &Info,
	c2s_states: &BTreeMap<String, StateSpec>,
	s2c_states: &BTreeMap<String, StateSpec>,
) -> TokenStream {
	let (c2s_module, c2s_enum_variants) = generate_module_and_variants(info, c2s_states, "c2s");
	let (s2c_module, s2c_enum_variants) = generate_module_and_variants(info, s2c_states, "s2c");

	quote! {
		pub mod c2s {
			#c2s_module
		}
		pub mod s2c {
			#s2c_module
		}

		pub enum C2S {
			LegacyPing,
			Handshake(crate::handshake::Handshake),
			#( #c2s_enum_variants )*
		}
		pub enum S2C {
			LegacyPingResponse(crate::legacy::LegacyPingResponse),
			#( #s2c_enum_variants )*
		}
	}
}

fn generate_module_and_variants(
	info: &Info,
	states: &BTreeMap<String, StateSpec>,
	direction: &str,
) -> (TokenStream, Vec<TokenStream>) {
	let direction = direction.as_ident();

	let mut module = TokenStream::new();
	let mut enum_variants = Vec::new();

	for (name, state) in states {
		module.extend(state_module::gen_state_module(info, name, state));

		let feature = gen_feature_cfg(&state.feature);
		let variant_name = to_pascal_case(name).as_ident();
		let state_module = name.as_ident();
		let state_enum_name = state_enum_name(name).as_ident();
		enum_variants.push(quote! {
			#feature
			#variant_name(#direction::#state_module::#state_enum_name),
		});
	}

	(module, enum_variants)
}

fn state_enum_name(state_name: &str) -> String {
	format!("{}Packet", to_pascal_case(state_name))
}
