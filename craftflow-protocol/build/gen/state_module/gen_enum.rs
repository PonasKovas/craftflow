use crate::build::{state_spec::EnumSpec, AsIdent, Info};
use proc_macro2::TokenStream;
use quote::quote;

pub fn gen_enum(info: &Info, name: &str, spec: &EnumSpec) -> Vec<TokenStream> {
	let mut items = Vec::new();
	let enum_name = name.as_ident();

	quote! {
		pub struct #enum_name {

		}
	};

	items
}
