use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{Data, Fields};

pub fn gen_impl_code(data: &Data) -> TokenStream {
	match data {
		Data::Struct(data) => {
			let members = data.fields.members();

			quote! {
				Self::Target {
					#(#members: ShallowClone::shallow_clone(&self.#members)),*
				}
			}
		}
		Data::Enum(data) => {
			let variants = data.variants.iter().map(|variant| {
				let variant_name = &variant.ident;
				match &variant.fields {
					Fields::Named(fields_named) => {
						let fields = fields_named.named.iter().map(|field| {
							let field_name = &field.ident;
							quote! { #field_name: ShallowClone::shallow_clone(#field_name) }
						});

						let fields_pat = fields_named.named.iter().map(|field| {
							let field_name = &field.ident;
							quote! { #field_name }
						});

						quote! {
							Self::#variant_name { #(#fields_pat),* } => Self::Target::#variant_name { #(#fields),* }
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
							Self::#variant_name ( #(#fields),* ) =>
								Self::Target::#variant_name ( #(ShallowClone::shallow_clone(#fields)),* )
						}
					}
					Fields::Unit => quote! {
					   Self::#variant_name => Self::Target::#variant_name
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
