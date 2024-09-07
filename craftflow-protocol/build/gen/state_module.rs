use crate::build::{
	gen::{feature_cfg::gen_feature_cfg, state_enum_name},
	state_spec::{SpecItem, StateSpec},
	AsIdent, Info,
};
use gen_enum::gen_enum;
use gen_packet::gen_packet;
use gen_struct::gen_struct;
use proc_macro2::TokenStream;
use quote::quote;

pub mod fields;
mod gen_enum;
mod gen_packet;
mod gen_struct;

// Generates the state module containing the state enum and all packets/structs/enums with their impls
//
// pub mod my_state {
//     pub enum MyStatePacket {
//         MyPacket(MyPacket),
//         Another(Another),
//     }
//     pub struct MyPacket { ... }
//     pub struct Another { ... }
//     ...
// }
pub fn gen_state_module(
	info: &Info,
	direction: &str,
	state_name: &str,
	state_spec: &StateSpec,
) -> TokenStream {
	let module_name = state_name.to_lowercase().as_ident();
	let enum_name = state_enum_name(state_name).as_ident();

	let mut variants = Vec::new();
	for (item_name, item) in state_spec.items.iter() {
		if let SpecItem::Packet(packet) = item {
			let item_name = item_name.as_ident();
			let feature_cfg = packet.feature.as_ref().map(|f| gen_feature_cfg(f, true));

			variants.push(quote! {
				#feature_cfg #item_name(#item_name),
			});
		}
	}

	let mut generated = TokenStream::new();
	for (name, item) in &state_spec.items {
		let feature_cfg = item.feature().as_ref().map(|f| gen_feature_cfg(f, true));

		let item_gen = match item {
			SpecItem::Packet(item) => gen_packet(info, direction, state_name, name, item),
			SpecItem::Struct(item) => gen_struct(info, name, item.fields()),
			SpecItem::Enum(item) => gen_enum(info, name, item),
		};

		generated.extend(quote! {
			#(
				#feature_cfg
				#item_gen
			)*
		});
	}

	let feature_cfg = state_spec
		.feature
		.as_ref()
		.map(|f| gen_feature_cfg(f, true));
	quote! {
		#feature_cfg
		pub mod #module_name {
			use std::borrow::Borrow;
			use crate::datatypes::*;

			pub enum #enum_name {
				#(
					#variants
				)*
			}

			#generated
		}
	}
}
