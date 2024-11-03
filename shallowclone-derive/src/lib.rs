mod gen_impl_code;
mod get_extra_bounds;
mod get_target_type;

use gen_impl_code::gen_impl_code;
use get_extra_bounds::get_extra_bounds;
use get_target_type::get_target_type;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute};
use syn::{DeriveInput, GenericParam};

#[proc_macro_derive(ShallowClone, attributes(shallowclone))]
pub fn derive(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let target_type = get_target_type(&input);

	// cant use impl_generics from this because its completely opaque and we need to add
	// our 'shallowclone lifetime
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

	let extra_user_bounds = get_extra_bounds(&input);

	let extra_bounds = input.generics.params.iter().filter_map(|p| match p {
		GenericParam::Lifetime(lifetime_param) => {
			let lifetime = &lifetime_param.lifetime;
			Some(quote! { #lifetime: 'shallowclone, })
		}
		GenericParam::Type(type_param) => {
			let name = &type_param.ident;
			if has_shallow_clone_bound(&type_param.attrs) {
				Some(quote! { #name: ShallowClone<'shallowclone>, })
			} else {
				None
			}
		}
		GenericParam::Const(_const_param) => None,
	});
	let where_clause = match where_clause {
		Some(where_clause) => {
			quote! {
				#where_clause,
				#extra_user_bounds
				#(#extra_bounds)*
			}
		}
		None => quote! {
			where #extra_user_bounds
			#(#extra_bounds)*
		},
	};

	let impl_code = gen_impl_code(&input.data);
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

fn has_shallow_clone_bound(attrs: &[Attribute]) -> bool {
	attrs
		.iter()
		.any(|attr| attr.path().is_ident("shallowclone"))
}
