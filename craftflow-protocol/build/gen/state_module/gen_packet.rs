use crate::build::{state_spec::PacketSpec, AsIdent, Info};
use proc_macro2::TokenStream;
use quote::quote;

// Generates:
// * Packet definition
// * MinecraftProtocol implementation
// * Packet implementation
pub fn gen_packet(info: &Info, name: &str, spec: &PacketSpec) -> Vec<TokenStream> {
	let mut items = Vec::new();
	let packet_name = name.as_ident();

	// Packet definition

	let fields = spec.fields().gen_defs(true, true, true);
	items.push(quote! {
		pub struct #packet_name {
			#( #fields, )*
		}
	});

	// MinecraftProtocol implementation

	let fields_init = spec.fields().gen_defs(true, false, false);
	let read_fields = spec.fields().gen_minecraftprotocol_read(info);
	let fields = spec.fields().gen_defs(false, false, true);
	items.push(quote! {
		impl crate::MinecraftProtocol for #packet_name {
			fn read(___PROTOCOL_VERSION___: u32, ___SOURCE___: &mut impl std::io::Read) -> anyhow::Result<Self> {
				#( #[allow(unused_variables)] let #fields_init; )*

				#read_fields

				Ok(Self { #( #fields, )* })
			}

		}
	});

	items
}
