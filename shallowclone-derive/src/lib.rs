mod makeowned;
mod shallowclone;

use makeowned::gen_impl_makeowned;
use proc_macro::TokenStream;
use quote::quote;
use shallowclone::attributes::ShallowCloneAttributes;
use shallowclone::gen_impl_shallowclone;
use syn::parse_macro_input;
use syn::{DeriveInput, GenericParam};

#[proc_macro_derive(ShallowClone, attributes(shallowclone))]
pub fn derive_shallowclone(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let is_cow = ShallowCloneAttributes::parse_item(&input.attrs).cow;

	let target_type = shallowclone::get_target_type(&input);

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
			let skip = ShallowCloneAttributes::parse_generic(&type_param.attrs).skip;

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

	let impl_code = gen_impl_shallowclone(&input.ident, &input.data, is_cow);
	let ident = &input.ident;
	quote! {
		impl<'shallowclone, #(#impl_generics),*> ShallowClone<'shallowclone> for #ident #type_generics #where_clause {
			type Target = #target_type;

			fn shallow_clone(&'shallowclone self) -> <Self as ShallowClone<'shallowclone>>::Target {
				#impl_code
			}
		}
	}.into()
}

#[proc_macro_derive(MakeOwned, attributes(makeowned))]
pub fn derive_makeowned(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let target_type = makeowned::get_target_type(&input);

	let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

	let extra_bounds = input.generics.params.iter().filter_map(|p| match p {
		GenericParam::Lifetime(_lifetime_param) => None,
		GenericParam::Type(type_param) => {
			let name = &type_param.ident;
			// the <T as MakeOwned>::Owned must be bound by the same bounds as T
			// since we are gonna be using it in place of T
			let extra_bounds = &type_param.bounds;

			Some(quote! { #name: MakeOwned, <#name as MakeOwned>::Owned: #extra_bounds, })
		}
		GenericParam::Const(_const_param) => None,
	});
	// We should also duplicate all bounds in the where clause, replacing T with <T as MakeOwned>::Owned
	// but thats quite complicated, so for now we just dont support where clauses
	//
	// Another solution would be to use a #[makeowned(bound = "")] attribute to specify the bounds
	// instead of trying to parse the where clause, and this might actually be a better solution
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

	let impl_code = gen_impl_makeowned(&input.ident, &input.data);
	let ident = &input.ident;
	quote! {
		impl #impl_generics MakeOwned for #ident #type_generics #where_clause {
			type Owned = #target_type;

			fn make_owned(self) -> <Self as MakeOwned>::Owned {
				#impl_code
			}
		}
	}
	.into()
}
