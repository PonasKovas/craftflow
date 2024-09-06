use crate::build::{state_spec::StructSpec, AsIdent, Info};
use proc_macro2::TokenStream;
use quote::quote;

pub fn gen_struct(info: &Info, name: &str, spec: &StructSpec) -> Vec<TokenStream> {
	let mut items = Vec::new();
	let struct_name = name.as_ident();

	quote! {
		pub struct #struct_name {

		}
	};

	items
}
