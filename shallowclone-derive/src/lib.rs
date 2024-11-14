mod gen_impl_code;
mod get_target_type;
mod parse_attrs;

use gen_impl_code::gen_impl_code;
use get_target_type::get_target_type;
use parse_attrs::Attributes;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::{DeriveInput, GenericParam};

#[proc_macro_derive(ShallowClone, attributes(shallowclone))]
pub fn derive(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let is_cow = Attributes::parse_item(&input.attrs).cow;

	let target_type = get_target_type(&input);

	// cant use impl_generics from this because its completely opaque and we need to add
	// our 'shallowclone lifetime :(
	let (_, type_generics, where_clause) = input.generics.split_for_impl();

	// we cant use input.generics.params directly because we need to remove the #[shallowclone]
	// attributes
	// this basically prints the generics without attributes
	let impl_generics = input.generics.params.iter().map(|p| match p {
		GenericParam::Lifetime(lifetime_param) => {
			let lifetime = &lifetime_param.lifetime;
			let bounds = &lifetime_param.bounds;
			quote! { #lifetime: #bounds }
		}
		GenericParam::Type(type_param) => {
			let name = &type_param.ident;
			let bounds = &type_param.bounds;
			quote! { #name: #bounds }
		}
		GenericParam::Const(const_param) => {
			let name = &const_param.ident;
			let ty = &const_param.ty;
			quote! { const #name: #ty }
		}
	});

	let extra_bounds = input.generics.params.iter().filter_map(|p| match p {
		GenericParam::Lifetime(lifetime_param) => {
			let lifetime = &lifetime_param.lifetime;
			Some(quote! { #lifetime: 'shallowclone, })
		}
		GenericParam::Type(type_param) => {
			let skip = Attributes::parse_generic(&type_param.attrs).skip;

			if skip {
				None
			} else {
				let name = &type_param.ident;
				Some(quote! { #name: ShallowClone<'shallowclone>, })
			}
		}
		GenericParam::Const(_const_param) => None,
	});
	let where_clause = match where_clause {
		Some(where_clause) => {
			quote! {
				#where_clause,
				#(#extra_bounds)*
			}
		}
		None => quote! {
			where #(#extra_bounds)*
		},
	};

	let impl_code = gen_impl_code(&input.ident, &input.data, is_cow);
	let ident = &input.ident;
	quote! {
		impl<'shallowclone, #(#impl_generics),*> ShallowClone<'shallowclone> for #ident #type_generics #where_clause {
			type Target = #target_type;

			fn shallow_clone(&'shallowclone self) -> Self::Target {
				#impl_code
			}
		}
	}.into()
}
