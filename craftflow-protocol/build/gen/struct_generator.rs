use super::{
	feature::Feature,
	field::{Field, FieldFormat},
	fields_container::FieldsContainer,
};
use crate::build::{
	gen::{feature::FeatureCfgOptions, field::FieldGenOptions},
	version_bounds::{Bounds, BoundsMethods},
};
use indexmap::IndexMap;
use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub struct StructGenerator {
	pub name: Ident,
	pub feature: Option<Feature>,
	pub fields: FieldsContainer,
}

impl StructGenerator {
	/// Generates a struct definition and a MinecraftProtocol implementation
	pub fn gen(&self) -> TokenStream {
		let feature_cfg = self.feature.as_ref().map(|f| {
			f.gen_cfg(FeatureCfgOptions {
				negative: false,
				with_doc_note: true,
			})
		});
		let struct_name = &self.name;

		let fields = self.fields.gen_definition();

		let read_impl = self.gen_read_impl();
		let write_impl = self.gen_write_impl();

		quote! {
			#feature_cfg
			pub struct #struct_name {
				#fields
			}

			impl crate::MinecraftProtocol for #struct_name {
				fn read(
					#[allow(non_snake_case)] ___PROTOCOL_VERSION___: u32,
					#[allow(non_snake_case)] ___INPUT___: &mut impl std::io::Read
				) -> anyhow::Result<Self> {
					#read_impl
				}
				fn write(
					&self,
					#[allow(non_snake_case)] ___PROTOCOL_VERSION___: u32,
					#[allow(non_snake_case)] ___OUTPUT___: &mut impl std::io::Write
				) -> anyhow::Result<usize> {
					#write_impl
				}
			}
		}
	}
	fn gen_read_impl(&self) -> TokenStream {
		let mut result = TokenStream::new();

		// This creates a variable for every field
		// reading from the input
		result.extend(self.fields.gen_read_impl());

		// So now all that's left is to construct the struct
		// with those variables
		let constructor = self.fields.gen_constructor();
		result.extend(quote! {
			Ok(Self{#constructor})
		});

		result
	}
	fn gen_write_impl(&self) -> TokenStream {
		// start with the written bytes variable
		let mut result = quote! {
			#[allow(non_snake_case)]
			let mut ___WRITTEN_BYTES___: usize = 0;
		};

		// Now put all fields into their variables (by destructuring the struct or by using their default
		// values if the field requires a feature that is not enabled)
		for field in &self.fields.fields {
			let field_name = &field.name;

			result.extend(match &field.feature {
				None => quote! { let #field_name = &self.#field_name; },
				Some(feature) => {
					let feature_cfg = feature.feature.gen_cfg(FeatureCfgOptions {
						negative: false,
						with_doc_note: false,
					});
					let not_feature_cfg = feature.feature.gen_cfg(FeatureCfgOptions {
						negative: true,
						with_doc_note: false,
					});

					let default = &feature.default_value;

					quote! {
						#feature_cfg
						#[allow(unused_variables)]
						let #field_name = &self.#field_name;

						#not_feature_cfg
						#[allow(unused_variables)]
						let #field_name = { #default }.borrow();
					}
				}
			});
		}

		// Write all the fields
		result.extend(self.fields.gen_write_impl());

		// Return the written bytes
		result.extend(quote! {
			Ok(___WRITTEN_BYTES___)
		});

		result
	}
}
