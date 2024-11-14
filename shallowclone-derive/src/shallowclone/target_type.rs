use crate::shallowclone::attributes::ShallowCloneAttributes;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, GenericParam};

/// Generates `Name<generics>` with generics changed accordingly
pub fn get_target_type(input: &DeriveInput) -> TokenStream {
	let name = &input.ident;
	let generics = input.generics.params.iter().map(|g| match g {
		GenericParam::Type(type_param) => {
			let name = &type_param.ident;
			let skip = ShallowCloneAttributes::parse_generic(&type_param.attrs).skip;
			if skip {
				quote! { #name }
			} else {
				quote! { <#name as ShallowClone<'shallowclone>>::Target }
			}
		}
		GenericParam::Lifetime(lifetime_param) => {
			let name = &lifetime_param.lifetime;
			let skip = ShallowCloneAttributes::parse_generic(&lifetime_param.attrs).skip;
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
