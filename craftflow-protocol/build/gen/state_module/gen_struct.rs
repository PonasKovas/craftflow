use super::fields::Fields;
use crate::build::{
	gen::feature_cfg::{gen_feature_cfg, gen_not_feature_cfg},
	state_spec::Data,
	AsIdent, AsTokenStream, Info,
};
use proc_macro2::TokenStream;
use quote::quote;

// Generates:
// * Struct definition
// * MinecraftProtocol implementation
pub fn gen_struct(info: &Info, name: &str, fields: Fields) -> Vec<TokenStream> {
	let mut items = Vec::new();
	let struct_name = name.as_ident();

	// Struct definition

	let field_defs = fields.gen(true, true, true, false);
	items.push(quote! {
		pub struct #struct_name {
			#( #field_defs, )*
		}
	});

	// MinecraftProtocol implementation

	let read_field_inits = fields.gen(true, false, false, true);
	let read_fields = fields.gen_minecraftprotocol_read(info);
	let construction_fields = fields.gen(false, false, true, false);

	let write_field_inits = fields.data.iter().map(|(name, field)| {
		// field_name = &self.field_name OR default
		let name = name.as_ident();
		match field {
			Data::Normal(_) => quote! { let #name = &self.#name; },
			Data::RequiresFeature {
				feature,
				data_type: _,
				default,
			} => {
				let feature_cfg = gen_feature_cfg(feature, false);
				let not_feature_cfg = gen_not_feature_cfg(feature, false);

				let default = default.as_tokenstream();
				quote! {
					#feature_cfg
					#[allow(unused_variables)]
					let #name = &self.#name;

					#not_feature_cfg
					#[allow(unused_variables)]
					let #name = { #default }.borrow();
				}
			}
		}
	});
	let write_fields = fields.gen_minecraftprotocol_write(info);

	items.push(quote! {
		impl crate::MinecraftProtocol for #struct_name {
			fn read(#[allow(non_snake_case)] ___PROTOCOL_VERSION___: u32, #[allow(non_snake_case)] ___INPUT___: &mut impl std::io::Read) -> anyhow::Result<Self> {
				#( #[allow(unused_variables, unused_assignments, unused_mut)] let mut #read_field_inits; )*

				#read_fields

				Ok(Self { #( #construction_fields, )* })
			}

			fn write(&self, #[allow(non_snake_case)] ___PROTOCOL_VERSION___: u32, #[allow(non_snake_case)] ___OUTPUT___: &mut impl std::io::Write) -> anyhow::Result<usize> {
				#[allow(non_snake_case)]
				let mut ___WRITTEN_BYTES___: usize = 0;

				#( #write_field_inits )*

				#write_fields

				Ok(___WRITTEN_BYTES___)
			}
		}
	});

	items
}
