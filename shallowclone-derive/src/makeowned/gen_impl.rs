use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{Data, Fields, Index};

pub fn gen_impl_makeowned(item_name: &Ident, data: &Data) -> TokenStream {
	match data {
		Data::Struct(data) => {
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

fn gen_fields(fields: &Fields, is_enum: bool) -> TokenStream {
	let inner = fields.iter().enumerate().map(|(i, field)| {
		let field_ident = match (is_enum, &field.ident) {
			(false, Some(ident)) => quote! { self.#ident },
			(true, Some(ident)) => quote! { #ident },
			(false, None) => {
				let i = Index::from(i);
				quote! { self.#i }
			}
			(true, None) => {
				let x = Ident::new(&format!("f{i}"), Span::call_site());
				quote! { #x }
			}
		};

		let value = quote! { MakeOwned::make_owned(#field_ident) };

		match &field.ident {
			Some(field_name) => quote! { #field_name: #value },
			None => quote! { #value },
		}
	});

	quote! {
		#(#inner),*
	}
}
