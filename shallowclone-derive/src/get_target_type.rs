use proc_macro2::TokenStream;
use quote::quote;
use std::str::FromStr;
use syn::{DeriveInput, Expr, GenericParam, Lit, MetaNameValue};

pub fn get_target_type(input: &DeriveInput) -> TokenStream {
	match input
		.attrs
		.iter()
		.find(|attr| attr.path().is_ident("shallowclone"))
	{
		None => {
			// allowed to not have this attribute only when there are no generics
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
			let target_type: MetaNameValue = attr.parse_args().unwrap();
			if target_type.path.is_ident("target") {
				if let Expr::Lit(expr) = target_type.value {
					if let Lit::Str(lit_str) = expr.lit {
						return TokenStream::from_str(&lit_str.value()).unwrap();
					}
				}
			}
			panic!("invalid #[shallowclone(target = \"...\")] attribute");
		}
	}
}
