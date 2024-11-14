use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, GenericParam};

/// Generates `Name<generics>` with generics changed accordingly
pub fn get_target_type(input: &DeriveInput) -> TokenStream {
	let name = &input.ident;
	let generics = input.generics.params.iter().map(|g| match g {
		GenericParam::Type(type_param) => {
			let name = &type_param.ident;

			quote! { <#name as MakeOwned>::Owned }
		}
		GenericParam::Lifetime(_lifetime_param) => quote! { 'static },
		GenericParam::Const(const_param) => quote! { #const_param, },
	});

	quote! { #name< #(#generics),* > }
}
