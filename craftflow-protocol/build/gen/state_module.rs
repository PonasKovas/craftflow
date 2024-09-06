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
pub fn gen_state_module(info: &Info, state_name: &str, state_spec: &StateSpec) -> TokenStream {
	let module_name = state_name.to_lowercase().as_ident();
	let enum_name = state_enum_name(state_name).as_ident();

	let mut variants = Vec::new();
	for (item_name, item) in state_spec.items.iter() {
		if let SpecItem::Packet(packet) = item {
			let item_name = item_name.as_ident();
			let feature = gen_feature_cfg(&packet.feature);

			variants.push(quote! {
				#feature #item_name(#item_name),
			});
		}
	}

	let mut generated = TokenStream::new();
	for (name, item) in &state_spec.items {
		let feature = gen_feature_cfg(item.feature());

		let item_gen = match item {
			SpecItem::Packet(item) => gen_packet(info, name, item),
			SpecItem::Struct(item) => gen_struct(info, name, item),
			SpecItem::Enum(item) => gen_enum(info, name, item),
		};

		generated.extend(quote! {
			#(
				#feature
				#item_gen
			)*
		});
	}

	let feature = gen_feature_cfg(&state_spec.feature);
	quote! {
		#feature
		pub mod #module_name {
			pub enum #enum_name {
				#(
					#variants
				)*
			}

			#generated
		}
	}
}
