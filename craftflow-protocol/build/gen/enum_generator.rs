use super::{
	custom_format::CustomFormat, feature::Feature, fields_container::FieldsContainer,
	version_dependent::VersionDependent,
};
use crate::build::{gen::feature::FeatureCfgOptions, info_file::Info};
use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub struct EnumGenerator {
	pub name: Ident,
	pub feature: Option<Feature>,
	pub variants: Vec<Variant>,
	pub tag_format: VersionDependent<CustomFormat>,
}

pub struct Variant {
	pub name: Ident,
	pub feature: Option<Feature>,
	pub tags: VersionDependent<TokenStream>,
	pub fields: FieldsContainer,
}

impl EnumGenerator {
	/// Generates the enum definition and MinecraftProtocol implementation
	pub fn gen(&self, info: &Info) -> TokenStream {
		let feature_cfg = self.feature.as_ref().map(|f| {
			f.gen_cfg(FeatureCfgOptions {
				negative: false,
				with_doc_note: true,
			})
		});
		let enum_name = &self.name;

		let mut variants = TokenStream::new();
		for variant in &self.variants {
			let variant_name = &variant.name;

			let fields = variant.fields.gen_definition(false);

			let feature_cfg = variant.feature.as_ref().map(|f| {
				f.gen_cfg(FeatureCfgOptions {
					negative: false,
					with_doc_note: true,
				})
			});
			variants.extend(quote! {
				#feature_cfg
				#variant_name {#fields},
			});
		}

		// If any of the enum variants require a feature, add an extra variant _Unsupported
		// to which the read operations would fallback to if you dont have the feature enabled
		// and read that variant
		variants.extend(quote! {
			#[doc = "Fallback variant for when you read a variant that you do not have the feature enabled for."]
			#[doc = "Even if this enum does not have any variants that require features, in theory, one"]
			#[doc = "could be added in the future."]
			_Unsupported,
		});

		let read_impl = self.gen_read_impl(info);
		let write_impl = self.gen_write_impl(info);

		quote! {
			#[derive(Debug, Clone, PartialEq)]
			#feature_cfg
			pub enum #enum_name {
				#variants
			}

			#feature_cfg
			impl crate::MinecraftProtocol for #enum_name {
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
	fn gen_read_impl(&self, info: &Info) -> TokenStream {
		let mut result = TokenStream::new();

		// first we have to read the tag, according to the protocol format
		let tag_match = self.tag_format.gen_protocol_version_match(info, |tag_format| {
			match &tag_format.custom_read {
				Some(custom_read) => {
					let read_as = &custom_read.read_as;
					let read = &custom_read.read;

					quote! {
						{
							#[allow(non_snake_case)]
							let THIS = <#read_as as crate::MinecraftProtocol>::read(___PROTOCOL_VERSION___, ___INPUT___)?;
							#read
						}
					}
				}
				None => {
					// default read as varint
					quote! {
						<crate::datatypes::VarInt as crate::MinecraftProtocol>::read(___PROTOCOL_VERSION___, ___INPUT___)?.0
					}
				}
			}
		});
		result.extend(quote! {
			#[allow(non_snake_case)]
			let ___TAG___ = #tag_match;
		});

		// now we have to match the tag to find which variant it is
		// try each variant sequentially
		for variant in &self.variants {
			let variant_name = &variant.name;

			// Prepare to read this variant in case the tag matches
			let mut read_variant = TokenStream::new();

			// This creates a variable for every field
			// reading from the input
			read_variant.extend(variant.fields.gen_read_impl(info));

			// Construct the variant
			let constructor = variant.fields.gen_constructor();
			read_variant.extend(quote! {
				return Ok(Self::#variant_name{#constructor});
			});

			let variant_match = variant.tags.gen_protocol_version_match(info, |tag| {
				quote! {
					if ___TAG___ == #tag {
						// Found it!
						// continue reading this variant now
						#read_variant
					}
				}
			});

			result.extend(variant_match);
		}

		// If couldn't match any variants and made it here
		result.extend(quote! {
			anyhow::bail!("couldnt match enum tag: {:?}", ___TAG___);
		});

		result
	}
	fn gen_write_impl(&self, info: &Info) -> TokenStream {
		let mut self_match = TokenStream::new();

		for variant in &self.variants {
			let variant_name = &variant.name;

			let destructor = variant.fields.gen_constructor();

			let get_tag = variant
				.tags
				.gen_protocol_version_match(info, |tag| tag.clone());

			let write_tag = self
				.tag_format
				.gen_protocol_version_match(info, |tag_format| match &tag_format.custom_write {
					None => quote! {
						___WRITTEN_BYTES___ += crate::MinecraftProtocol::write(
							&crate::datatypes::VarInt(___TAG___),
							___PROTOCOL_VERSION___,
							___OUTPUT___
						)?;
					},
					Some(custom_write) => quote! {
						___WRITTEN_BYTES___ += {
							#[allow(non_snake_case, unused_variables)]
							let THIS = ___TAG___;
							crate::MinecraftProtocol::write(
								#[allow(unused_braces)] { #custom_write },
								___PROTOCOL_VERSION___,
								___OUTPUT___
							)?
						};
					},
				});

			let write_fields = variant.fields.gen_write_impl(info);

			self_match.extend(quote! {
				Self::#variant_name { #destructor } => {
					// get the tag
					#[allow(non_snake_case)]
					let ___TAG___ = #get_tag;

					// write the tag
					#write_tag

					// write the fields
					#write_fields
				},
			});
		}

		quote! {
			#[allow(non_snake_case)]
			let mut ___WRITTEN_BYTES___: usize = 0;

			match self {
				#self_match
				Self::_Unsupported => panic!("Broo are you being serious right now... You cant send the `_Unsupported` variant. This is not what it's for. Please tell me you're joking."),
			}

			Ok(___WRITTEN_BYTES___)
		}
	}
}
