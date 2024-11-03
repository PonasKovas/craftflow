use proc_macro2::TokenStream;
use quote::quote;
use std::str::FromStr;
use syn::DeriveInput;

pub fn get_extra_bounds(input: &DeriveInput) -> TokenStream {
	let mut extra_bounds = Vec::new();

	for attr in &input.attrs {
		if !attr.path().is_ident("shallowclone") {
			continue;
		}

		let parsed: syn::MetaNameValue = attr.parse_args().unwrap();
		if !parsed.path.is_ident("bound") {
			continue;
		}

		if let syn::Expr::Lit(expr) = parsed.value {
			if let syn::Lit::Str(lit_str) = expr.lit {
				extra_bounds.push(TokenStream::from_str(&lit_str.value()).unwrap());
				continue;
			}
		}
		panic!("invalid bound in #[shallowclone(bound = \"...\")]");
	}

	quote! { #(#extra_bounds,)* }
}
