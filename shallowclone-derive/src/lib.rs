mod gen_impl_code;
mod get_target_type;

use gen_impl_code::gen_impl_code;
use get_target_type::get_target_type;
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, Attribute};
use syn::{DeriveInput, GenericParam};

#[proc_macro_derive(ShallowClone, attributes(shallowclone))]
pub fn derive(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	// Extract the target type from attribute
	let target_type = get_target_type(&input);

	let impl_generics = input.generics.params.iter().map(|g| match g {
		GenericParam::Type(type_param) => {
			let name = &type_param.ident;
			let bounds = type_param.bounds.iter();
			if has_shallow_clone_bound(&type_param.attrs) {
				quote! { #name: ShallowClone<'shallowclone> #( + #bounds)*, }
			} else {
				quote! { #name: #( #bounds + )*, }
			}
		}
		GenericParam::Lifetime(lifetime_param) => quote! { #lifetime_param, },
		GenericParam::Const(const_param) => quote! { #const_param, },
	});

	let type_generics = input.generics.params.iter().map(|p| match p {
		GenericParam::Lifetime(lifetime_param) => {
			let name = &lifetime_param.lifetime;
			quote! { #name, }
		}
		GenericParam::Type(type_param) => {
			let name = &type_param.ident;
			quote! { #name, }
		}
		GenericParam::Const(const_param) => {
			let name = &const_param.ident;
			quote! { #name, }
		}
	});

	let extra_lifetime_bounds = input.generics.params.iter().filter_map(|p| {
		if let GenericParam::Lifetime(lifetime_param) = p {
			let lifetime = &lifetime_param.lifetime;
			Some(quote! { #lifetime: 'shallowclone, })
		} else {
			None
		}
	});
	let where_clause = match &input.generics.where_clause {
		Some(where_clause) => {
			quote! {
				#where_clause,
				#(#extra_lifetime_bounds)*
			}
		}
		None => quote! {
			where #(#extra_lifetime_bounds)*
		},
	};

	let impl_code = gen_impl_code(&input.data);
	let ident = &input.ident;
	quote! {
		impl<'shallowclone, #(#impl_generics)*> ShallowClone<'shallowclone> for #ident <#(#type_generics)*> #where_clause {
			type Target = #target_type;

			fn shallow_clone(&'shallowclone self) -> Self::Target {
				#impl_code
			}
		}
	}.into()
}

fn has_shallow_clone_bound(attrs: &[Attribute]) -> bool {
	attrs
		.iter()
		.any(|attr| attr.path().is_ident("shallowclone"))
}
