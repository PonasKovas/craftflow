use super::{
	custom_format::CustomFormat,
	feature::Feature,
	field::{Field, FieldFormat},
	fields_container::FieldsContainer,
};
use crate::build::{gen::field::FieldGenOptions, version_bounds::Bounds};
use indexmap::IndexMap;
use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub struct EnumGenerator {
	pub name: Ident,
	pub feature: Option<Feature>,
	pub variants: Vec<Variant>,
	pub tag_format: IndexMap<Vec<Bounds>, CustomFormat>,
}

pub struct Variant {
	pub name: Ident,
	pub tags: IndexMap<Vec<Bounds>, TokenStream>,
	pub fields: FieldsContainer,
}

impl EnumGenerator {
	/// Generates the enum definition and MinecraftProtocol implementation
	pub fn gen(&self) -> TokenStream {
		return quote! {};
		let enum_name = &self.name;

		let variants = self.variants.iter().map(|v| {
			let variant_name = &v.name;
			let fields_def = v.fields.gen_definition();
			quote! { #variant_name #fields_def }
		});

		quote! {
			pub enum #enum_name {
				#( #variants, )*
			}
		}
	}
}
