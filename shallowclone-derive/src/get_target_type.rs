use crate::has_shallow_clone_bound;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, GenericParam};
// Commented out while we figure out whats wrong with rust
// https://github.com/rust-lang/rust/issues/132561

// pub fn get_target_type(input: &DeriveInput) -> TokenStream {
// 	let name = &input.ident;
// 	let generics = input.generics.params.iter().map(|g| match g {
// 		GenericParam::Type(type_param) => {
// 			let name = &type_param.ident;
// 			if has_shallow_clone_bound(&type_param.attrs) {
// 				quote! { #name::Target }
// 			} else {
// 				quote! { #name }
// 			}
// 		}
// 		GenericParam::Lifetime(lifetime_param) => {
// 			if has_shallow_clone_bound(&lifetime_param.attrs) {
// 				quote! { 'shallowclone }
// 			} else {
// 				quote! { #lifetime_param }
// 			}
// 		}
// 		GenericParam::Const(const_param) => quote! { #const_param, },
// 	});

// 	quote! { #name< #(#generics),* > }
// }

pub fn get_target_type(input: &DeriveInput) -> TokenStream {
	match input
		.attrs
		.iter()
		.find(|attr| attr.path().is_ident("shallowclone"))
	{
		None => {
			// allowed to not have this attribute only when there are no type generics
			if input
				.generics
				.params
				.iter()
				.any(|p| matches!(p, GenericParam::Type(_)))
			{
				panic!("missing #[shallowclone(target = \"...\")] attribute");
			}

			let generics = input.generics.params.iter().map(|p| match p {
				GenericParam::Lifetime(_) => quote! { 'shallowclone },
				GenericParam::Const(const_param) => {
					let name = &const_param.ident;
					quote! { #name }
				}
				_ => unreachable!(),
			});
			let ident = &input.ident;
			quote! { #ident < #(#generics),* > }
		}
		Some(attr) => {
			use std::str::FromStr;

			let target_type: syn::MetaNameValue = attr.parse_args().unwrap();
			if target_type.path.is_ident("target") {
				if let syn::Expr::Lit(expr) = target_type.value {
					if let syn::Lit::Str(lit_str) = expr.lit {
						return TokenStream::from_str(&lit_str.value()).unwrap();
					}
				}
			}
			panic!("invalid #[shallowclone(target = \"...\")] attribute");
		}
	}
}
