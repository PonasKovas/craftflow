use crate::DeriveType;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Fields, Index};

fn tuple_field(i: usize) -> Ident {
	Ident::new(&format!("f{i}"), Span::call_site())
}

pub fn gen_impl(derive_type: DeriveType, input: &DeriveInput) -> TokenStream {
	let item_name = &input.ident;

	match &input.data {
		Data::Struct(data) => {
			let inner = gen_fields(derive_type, &data.fields, false);

			match &data.fields {
				Fields::Named(_) => quote! {
					#item_name { #inner }
				},
				Fields::Unnamed(_) => quote! {
					#item_name ( #inner )
				},
				Fields::Unit => quote! { #item_name },
			}
		}
		Data::Enum(data) => {
			let variants = data.variants.iter().map(|variant| {
				let variant_name = &variant.ident;

				let inner = gen_fields(derive_type, &variant.fields, true);

				match &variant.fields {
					Fields::Named(fields_named) => {
						let fields_pat = fields_named.named.iter().map(|field| {
							let field_name = &field.ident;
							quote! { #field_name }
						});

						quote! {
							Self::#variant_name { #(#fields_pat),* } => #item_name::#variant_name { #inner }
						}
					}
					Fields::Unnamed(fields_unnamed) => {
						let fields = fields_unnamed
							.unnamed
							.iter()
							.enumerate()
							.map(|(i, _)| tuple_field(i))
							.collect::<Vec<_>>();
						quote! {
							Self::#variant_name ( #(#fields),* ) => #item_name::#variant_name ( #inner )
						}
					}
					Fields::Unit => quote! {
					   Self::#variant_name => #item_name::#variant_name
					},
				}
			});

			quote! {
				match self {
					#(#variants),*
				}
			}
		}
		Data::Union(_) => unimplemented!(),
	}
}

fn gen_fields(derive_type: DeriveType, fields: &Fields, is_enum: bool) -> TokenStream {
	let inner = fields.iter().enumerate().map(|(i, field)| {
		let field_ident = match (derive_type, is_enum, &field.ident) {
			(_, true, Some(ident)) => quote! { #ident },
			(DeriveType::ShallowClone, false, Some(ident)) => quote! { &self.#ident },
			(DeriveType::MakeOwned, false, Some(ident)) => quote! { self.#ident },

			(DeriveType::ShallowClone, false, None) => {
				let i = Index::from(i);
				quote! { &self.#i }
			}
			(DeriveType::MakeOwned, false, None) => {
				let i = Index::from(i);
				quote! { self.#i }
			}
			(_, true, None) => {
				let x = tuple_field(i);
				quote! { #x }
			}
		};

		let value = match derive_type {
			DeriveType::ShallowClone => quote! { ShallowClone::shallow_clone(#field_ident) },
			DeriveType::MakeOwned => quote! { MakeOwned::make_owned(#field_ident) },
		};

		match &field.ident {
			Some(field_name) => quote! { #field_name: #value },
			None => quote! { #value },
		}
	});

	quote! {
		#(#inner),*
	}
}
