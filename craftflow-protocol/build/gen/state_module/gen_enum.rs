use std::collections::BTreeMap;

use crate::build::{
	gen::feature_cfg::{gen_feature_cfg, gen_not_feature_cfg},
	state_spec::{Data, EnumSpec},
	util::{AsIdent, AsTokenStream},
	version_bounds::{Bounds, BoundsMethods},
	Info,
};
use proc_macro2::TokenStream;
use quote::quote;

// Generates:
// * Enum definition
// * MinecraftProtocol implementation
pub fn gen_enum(info: &Info, name: &str, spec: &EnumSpec) -> Vec<TokenStream> {
	let mut items = Vec::new();
	let enum_name = name.as_ident();

	// Enum definition

	let mut variants = Vec::new();
	for (variant_name, variant) in &spec.variants {
		let variant_name = variant_name.as_ident();

		let fields = match &variant.data {
			Some(data) => {
				let mut fields = Vec::new();
				for (field_name, field) in data {
					let field_name = field_name.as_ident();
					let data_type = field.datatype();

					match field {
						Data::Normal(_data_type) => {
							fields.push(quote! {
								#field_name : #data_type,
							});
						}
						Data::RequiresFeature {
							feature,
							data_type: _,
							default: _,
						} => {
							let feature_cfg = gen_feature_cfg(feature, true);
							fields.push(quote! {
								#feature_cfg
								#field_name : #data_type,
							});
						}
					}
				}

				Some(quote! { {
					#( #fields )*
				} })
			}
			None => None,
		};

		let feature_cfg = variant.feature.as_ref().map(|f| gen_feature_cfg(f, true));
		variants.push(quote! {
			#feature_cfg
			#variant_name #fields
		});
	}
	items.push(quote! {
		pub enum #enum_name {
			#( #variants, )*
		}
	});

	// MinecraftProtocol implementation

	let read_tag = match &spec.tag_format {
		None => {
			// use the default tag format - VarInt
			quote! { <crate::datatypes::VarInt as crate::MinecraftProtocol>::read(___PROTOCOL_VERSION___, ___INPUT___)? }
		}
		Some(format) => {
			// Special format
			let mut match_arms = Vec::new();

			for (bounds, format) in format.expand_shortcut() {
				let protocol_pattern = bounds.as_match_pattern();

				let right_side = match (format.read_as, format.read) {
					(None, Some(_)) | (Some(_), None) => {
						panic!("read and read_as have to come together. must not use them alone")
					}
					(None, None) => {
						// Normal read
						quote! {
							crate::MinecraftProtocol::read(___PROTOCOL_VERSION___, ___INPUT___)?
						}
					}
					(Some(read_as), Some(read)) => {
						// Special read
						let read_as = read_as.as_tokenstream();
						let read = read.as_tokenstream();

						quote! {
							{
								let tag: #read_as = crate::MinecraftProtocol::read(___PROTOCOL_VERSION___, ___INPUT___)?;
								#read
							}
						}
					}
				};

				match_arms.push(quote! {
					#protocol_pattern => #right_side,
				});
			}

			let unsupported_versions_arms = info.unsupported_versions_patterns();

			quote! {
				match ___PROTOCOL_VERSION___ {
					#( #match_arms )*

					// Match all versions that are not supported with the
					// current feature set so that we would get a compile error if
					// there are any possible uncovered protocol versions
					#( #unsupported_versions_arms => unreachable!("this protocol version is not supported with the current enabled feature set"), )*
				}
			}
		}
	};

	let mut read_match_arms = Vec::new();
	for (variant_name, variant) in &spec.variants {
		let variant_name = variant_name.as_ident();

		// we need to group by tags, not by version bounds here
		// could just change the state spec format to do
		// "tag": ["versions"] instead of ["versions"]: "tag"
		// but i want to make it consistent with other version-dependent things in the spec

		let mut tags_to_version_bounds = BTreeMap::new();
		for (bounds, tag) in variant.tag.expand_shortcut() {
			tags_to_version_bounds
				.entry(tag)
				.or_insert(Vec::new())
				.extend(bounds);
		}

		let right_side = match variant.fields() {
			Some(fields) => {
				let field_inits = fields.gen(true, false, false, true);
				let read_fields = fields.gen_minecraftprotocol_read(info);
				let construction_fields = fields.gen(false, false, true, false);

				quote! {
					#( #[allow(unused_variables, unused_assignments, unused_mut)] let mut #field_inits; )*

					#read_fields

					Self::#variant_name { #( #construction_fields, )* }
				}
			}
			None => quote! { Self::#variant_name },
		};

		for (tag, bounds) in tags_to_version_bounds {
			let protocol_pattern = bounds.as_match_pattern();
			let tag = tag.as_tokenstream();

			let unsupported_versions_arms = info.unsupported_versions_patterns();

			read_match_arms.push(quote! {
				#tag => match ___PROTOCOL_VERSION___ {
					#protocol_pattern => { #right_side },

					// Match all versions that are not supported with the
					// current feature set so that we would get a compile error if
					// there are any possible uncovered protocol versions
					#( #unsupported_versions_arms => unreachable!("this protocol version is not supported with the current enabled feature set"), )*
				},
			});
		}
	}

	let mut write_match_arms = Vec::new();
	for (variant_name, variant) in &spec.variants {
		let variant_name = variant_name.as_ident();

		let mut tag_arms = Vec::new();
		for (bounds, tag) in variant.tag.expand_shortcut() {
			let protocol_pattern = bounds.as_match_pattern();
			let tag = tag.as_tokenstream();

			tag_arms.push(quote! {
				#protocol_pattern => { #tag }.borrow(),
			});
		}

		let write_tag = match &spec.tag_format {
			None => {
				quote! { crate::MinecraftProtocol::write(tag, ___PROTOCOL_VERSION___, ___OUTPUT___)? }
			}
			Some(format) => {
				let mut protocol_arms = Vec::new();
				for (bounds, tag) in format.expand_shortcut() {
					let protocol_pattern = bounds.as_match_pattern();

					let write = match &tag.write {
						None => {
							quote! { crate::MinecraftProtocol::write(tag, ___PROTOCOL_VERSION___, ___OUTPUT___)? }
						}
						Some(write) => {
							let write = write.as_tokenstream();

							quote! { crate::MinecraftProtocol::write({ #write }.borrow(), ___PROTOCOL_VERSION___, ___OUTPUT___)? }
						}
					};

					protocol_arms.push(quote! {
						#protocol_pattern => #write,
					});
				}

				quote! {
					match ___PROTOCOL_VERSION___ {
						#( #protocol_arms )*
					}
				}
			}
		};

		let deconstruction_fields;
		let write_fields;

		match variant.fields() {
			None => {
				deconstruction_fields = None;
				write_fields = None;
			}
			Some(fields) => {
				deconstruction_fields = Some({
					let f = fields.gen(false, false, true, false);
					quote! { { #( #f, )* } }
				});

				let defaults = fields.data.iter().map(|(name, field)| {
					// field_name = default
					let name = name.as_ident();
					match field {
						Data::Normal(_) => quote! {},
						Data::RequiresFeature {
							feature,
							data_type: _,
							default,
						} => {
							let not_feature_cfg = gen_not_feature_cfg(feature, false);

							let default = default.as_tokenstream();
							quote! {
								#not_feature_cfg
								#[allow(unused_variables)]
								let #name = { #default }.borrow();
							}
						}
					}
				});
				let write_fields_inner = fields.gen_minecraftprotocol_write(info);

				write_fields = Some(quote! {
					#( #defaults )*

					#write_fields_inner
				})
			}
		}

		write_match_arms.push(quote! {
			Self::#variant_name #deconstruction_fields => {
				// write the tag
				{
					let tag = match ___PROTOCOL_VERSION___ {
						#( #tag_arms )*
					};
					___WRITTEN_BYTES___ += #write_tag;
				}


				// write the fields
				#write_fields
			},
		});
	}

	items.push(quote! {
		impl crate::MinecraftProtocol for #enum_name {
			fn read(#[allow(non_snake_case)] ___PROTOCOL_VERSION___: u32, #[allow(non_snake_case)] ___INPUT___: &mut impl std::io::Read) -> anyhow::Result<Self> {
				let tag = #read_tag;

				Ok(match tag {
					#( #read_match_arms )*
					other => anyhow::bail!("unexpected tag for #enum_name: {other:?}"),
				})
			}

			fn write(&self, #[allow(non_snake_case)] ___PROTOCOL_VERSION___: u32, #[allow(non_snake_case)] ___OUTPUT___: &mut impl std::io::Write) -> anyhow::Result<usize> {
				#[allow(non_snake_case)]
				let mut ___WRITTEN_BYTES___: usize = 0;

				match self {
					#( #write_match_arms )*
				}

				Ok(___WRITTEN_BYTES___)
			}
		}
	});

	items
}
