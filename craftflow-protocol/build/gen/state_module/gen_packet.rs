use super::gen_struct::gen_struct;
use crate::build::{
	gen::state_enum_name,
	state_spec::PacketSpec,
	util::{to_pascal_case, AsIdent},
	Info,
};
use proc_macro2::TokenStream;
use quote::quote;

// Generates:
// * Packet definition
// * MinecraftProtocol implementation
// * Packet implementation
pub fn gen_packet(
	info: &Info,
	direction: &str,
	state: &str,
	name: &str,
	spec: &PacketSpec,
) -> Vec<TokenStream> {
	// The definition and MinecraftProtocol implementation are identical as for structs
	let mut items = gen_struct(info, name, spec.fields());

	// Packet implementation

	let direction_enum = direction.to_uppercase().as_ident();
	let direction_module = direction.as_ident();
	let state_variant = to_pascal_case(state).as_ident();
	let state_module = state.as_ident();
	let state_enum = state_enum_name(state).as_ident();
	let packet_name = name.as_ident();
	items.push(quote! {
		impl crate::Packet for #packet_name {
			type Direction = crate::protocol::#direction_enum;

			fn into_packet_enum(self) -> Self::Direction {
				crate::protocol::#direction_enum::#state_variant(
					crate::protocol::#direction_module::#state_module::#state_enum::#packet_name(self)
				)
			}
		}
	});

	items
}
