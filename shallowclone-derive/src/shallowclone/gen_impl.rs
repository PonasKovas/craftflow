use crate::shallowclone::attributes::ShallowCloneAttributes;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{Data, DataEnum, Fields, Index};

pub fn gen_impl_shallowclone(item_name: &Ident, data: &Data, is_cow: bool) -> TokenStream {
	match data {
		Data::Struct(data) => {
			assert_eq!(is_cow, false, "struct cannot be cow");

			let inner = gen_fields(&data.fields, false);

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
			if is_cow {
				return gen_cow(item_name, data);
			}

			let variants = data.variants.iter().map(|variant| {
				let variant_name = &variant.ident;

				let inner = gen_fields(&variant.fields, true);

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
							.map(|(i, _)| Ident::new(&format!("f{i}"), Span::call_site()))
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

fn gen_cow(item_name: &Ident, data: &DataEnum) -> TokenStream {
	assert_eq!(
		data.variants.len(),
		2,
		"cow enums must have exactly 2 variants"
	);

	let owned = data
		.variants
		.iter()
		.find(|variant| ShallowCloneAttributes::parse_variant(&variant.attrs).owned)
		.expect("one variant must be marked as owned");
	let borrowed = data
		.variants
		.iter()
		.find(|variant| ShallowCloneAttributes::parse_variant(&variant.attrs).borrowed)
		.expect("one variant must be marked as borrowed");

	// in theory we don't need to be so strict but i want to enforce consistency
	assert_eq!(owned.fields.len(), 1, "cow variants must have only 1 field");
	assert_eq!(
		borrowed.fields.len(),
		1,
		"cow variants must have only 1 field"
	);

	let owned_field = &owned.fields.iter().next().unwrap();
	let borrowed_field = &borrowed.fields.iter().next().unwrap();

	assert_eq!(owned_field.ident, None, "cow variant field must be unnamed");
	assert_eq!(
		borrowed_field.ident, None,
		"cow variant field must be unnamed"
	);

	let owned_variant_name = &owned.ident;
	let borrowed_variant_name = &borrowed.ident;
	quote! {
		match self {
			#item_name::#owned_variant_name(inner) => #item_name::#borrowed_variant_name(&inner),
			#item_name::#borrowed_variant_name(inner) => #item_name::#borrowed_variant_name(inner),
		}
	}
}

fn gen_fields(fields: &Fields, is_enum: bool) -> TokenStream {
	let inner = fields.iter().enumerate().map(|(i, field)| {
		let field_ident = match (is_enum, &field.ident) {
			(false, Some(ident)) => quote! { &self.#ident },
			(true, Some(ident)) => quote! { #ident },
			(false, None) => {
				let i = Index::from(i);
				quote! { &self.#i }
			}
			(true, None) => {
				let x = Ident::new(&format!("f{i}"), Span::call_site());
				quote! { #x }
			}
		};

		let value = quote! { ShallowClone::shallow_clone(#field_ident) };

		match &field.ident {
			Some(field_name) => quote! { #field_name: #value },
			None => quote! { #value },
		}
	});

	quote! {
		#(#inner),*
	}
}
