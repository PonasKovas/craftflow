use crate::attributes;
use crate::DeriveType;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::{DeriveInput, GenericParam};

/// Generates `Name<generics>` with generics changed accordingly
pub fn get_target_type(input: &DeriveInput, derive_type: DeriveType) -> TokenStream {
	let generics = input.generics.params.iter().map(|generic| {
		let name: &dyn ToTokens = match generic {
			GenericParam::Type(type_param) => &type_param.ident,
			GenericParam::Lifetime(lifetime_param) => &lifetime_param.lifetime,
			// we always leave const generics as they are
			GenericParam::Const(const_param) => return quote! { #const_param },
		};

		if attributes::is_generic_skipped(derive_type, generic) {
			quote! { #name }
		} else {
			match (generic, derive_type) {
				(GenericParam::Type(_), DeriveType::ShallowClone) => {
					quote! { <#name as ShallowClone<'shallowclone>>::Target }
				}
				(GenericParam::Type(_), DeriveType::MakeOwned) => {
					quote! {<#name as MakeOwned>::Owned }
				}
				(GenericParam::Lifetime(_), DeriveType::ShallowClone) => quote! { 'shallowclone },
				(GenericParam::Lifetime(_), DeriveType::MakeOwned) => quote! { 'static },

				(GenericParam::Const(_), _) => unreachable!(),
			}
		}
	});

	let item_name = &input.ident;
	quote! { #item_name< #(#generics),* > }
}
