use crate::parse_attrs::Attributes;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, GenericParam};

pub fn get_target_type(input: &DeriveInput) -> TokenStream {
	let name = &input.ident;
	let generics = input.generics.params.iter().map(|g| match g {
		GenericParam::Type(type_param) => {
			let name = &type_param.ident;
			let skip = Attributes::parse_generic(&type_param.attrs).skip;
			if skip {
				quote! { #name }
			} else {
				quote! { #name::Target }
			}
		}
		GenericParam::Lifetime(lifetime_param) => {
			let name = &lifetime_param.lifetime;
			let skip = Attributes::parse_generic(&lifetime_param.attrs).skip;
			if skip {
				quote! { #name }
			} else {
				quote! { 'shallowclone }
			}
		}
		GenericParam::Const(const_param) => quote! { #const_param, },
	});

	quote! { #name< #(#generics),* > }
}
