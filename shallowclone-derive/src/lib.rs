mod attributes;
mod gen_impl;
mod target_type;

use gen_impl::gen_impl;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::quote;
use syn::parse_macro_input;
use syn::{DeriveInput, GenericParam};
use target_type::get_target_type;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum DeriveType {
	ShallowClone,
	MakeOwned,
}

#[proc_macro_error]
#[proc_macro_derive(ShallowClone, attributes(shallowclone))]
pub fn derive_shallowclone(input: TokenStream) -> TokenStream {
	derive(input, DeriveType::ShallowClone)
}

#[proc_macro_error]
#[proc_macro_derive(MakeOwned, attributes(makeowned))]
pub fn derive_makeowned(input: TokenStream) -> TokenStream {
	derive(input, DeriveType::MakeOwned)
}

fn derive(input: TokenStream, derive_type: DeriveType) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let ident = &input.ident;

	let target_type = match derive_type {
		DeriveType::ShallowClone => get_target_type(&input, derive_type),
		DeriveType::MakeOwned => get_target_type(&input, derive_type),
	};

	// i am actually at a loss of words. why do i have to reinvent the wheel every single
	// time i make a proc macro? why are there no abstractions for common stuff like DERIVING TRAITS
	// why tf does THIS FUCKING FUNCTION, which I would expect to be made for derive impls,
	// RETURN THE GENERICS WITH ATTRIBUTES WHICH DONT FUCKING COMPILE. AND WHY ARE THEY
	// OPAQUE, WHY CANT I ADD EXTRA GENERICS WHICH MY TRAIT MIGHT USE. WHAT THE FUCK IS THIS SHIT
	let (_, type_generics, where_clause) = input.generics.split_for_impl();

	let mut impl_generics = Vec::new();
	let mut extra_bounds = Vec::new();
	for generic in &input.generics.params {
		let skip = attributes::is_generic_skipped(derive_type, generic);

		match generic {
			GenericParam::Lifetime(lifetime_param) => {
				let lifetime = &lifetime_param.lifetime;
				let bounds = &lifetime_param.bounds;

				impl_generics.push(quote! { #lifetime: #bounds });

				if !skip && derive_type == DeriveType::ShallowClone {
					extra_bounds.push(quote! { #lifetime: 'shallowclone });
				}
			}
			GenericParam::Type(type_param) => {
				let name = &type_param.ident;
				let bounds = &type_param.bounds;

				impl_generics.push(quote! { #name: #bounds });

				if skip {
					if derive_type == DeriveType::MakeOwned {
						extra_bounds.push(quote! { #name: 'static });
					}
				} else {
					match derive_type {
						DeriveType::ShallowClone => {
							extra_bounds.push(quote! { #name: ShallowClone<'shallowclone> });
						}
						DeriveType::MakeOwned => {
							// the <T as MakeOwned>::Owned must be bound by the same bounds as T
							// since we are gonna be using it in place of T
							let orig_bounds = &type_param.bounds;

							extra_bounds.push(quote! { #name: MakeOwned });
							extra_bounds.push(quote! { <#name as MakeOwned>::Owned: #orig_bounds });
						}
					}
				}
			}
			GenericParam::Const(const_param) => {
				let name = &const_param.ident;
				let ty = &const_param.ty;

				impl_generics.push(quote! { const #name: #ty });
			}
		}
	}

	if derive_type == DeriveType::MakeOwned {
		// Since MakeOwned extends Clone, we want to implement it only if Self: Clone
		// but we cant just write this bound due to whatever reasons when there are lifetimes
		// because Self in this context comes with the specific lifetimes, and basically
		// Self<'static> ends up not included in the bound and then it in turn fucks up
		// the associated type MakeOwned::Owned, which must be Clone, but when we write
		// Self: Clone, the compiler starts assuming that Self<'static> is not Clone
		// (as its not included in the bound)
		// then you would think you could add another bound Self<'static>: Clone, but no,
		// compiler complains with a bunch of other weird errors.
		// Long story short this is most likely a bug in the compiler

		// get the generics with all lifetimes changed to 'any
		let generics = input.generics.params.iter().map(|param| match param {
			GenericParam::Lifetime(_) => quote! { 'any },
			GenericParam::Type(t) => {
				let ident = &t.ident;
				quote! { #ident }
			}
			GenericParam::Const(c) => {
				let ident = &c.ident;
				quote! { #ident }
			}
		});
		extra_bounds.push(quote! { for<'any> #ident <#(#generics),*>: Clone });
	}

	// For the MakeOwned:
	//   We should also duplicate all bounds in the where clause, replacing T with <T as MakeOwned>::Owned
	//   but thats quite complicated, so for now we just dont support where clauses
	//
	//   Another solution would be to use a #[shallowclone(bound = "")] attribute to specify the bounds
	//   instead of trying to parse the where clause. hard to tell without any specific cases in mind

	let where_clause = where_clause
		.map(|c| quote! { #c })
		.unwrap_or(quote! { where });
	let where_clause = quote! {
		#where_clause
		#(#extra_bounds),*
	};

	let impl_code = gen_impl(derive_type, &input);

	match derive_type {
		DeriveType::ShallowClone => quote! {
			impl<'shallowclone, #(#impl_generics),*> ShallowClone<'shallowclone> for #ident #type_generics
			#where_clause {
				type Target = #target_type;

				fn shallow_clone(&'shallowclone self) -> <Self as ShallowClone<'shallowclone>>::Target {
					#impl_code
				}
			}
		},
		DeriveType::MakeOwned => quote! {
			impl<#(#impl_generics),*> MakeOwned for #ident #type_generics
			#where_clause {
				type Owned = #target_type;

				fn make_owned(self) -> <Self as MakeOwned>::Owned {
					#impl_code
				}
			}
		},
	}
	.into()
}
