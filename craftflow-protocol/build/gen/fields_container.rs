use super::{
	custom_format::CustomFormat,
	feature::Feature,
	field::{Field, FieldFeatureReq, FieldFormat},
};
use crate::build::{
	gen::field::FieldGenOptions,
	state_spec::{self, Data, VersionDependent},
	util::{AsIdent, AsTokenStream},
	version_bounds::{Bounds, BoundsMethods},
};
use indexmap::IndexMap;
use proc_macro2::TokenStream;
use quote::quote;

pub struct FieldsContainer {
	pub fields: Vec<Field>,
	pub format: IndexMap<Vec<Bounds>, Vec<FieldFormat>>,
}

impl FieldsContainer {
	pub fn from_spec(
		data: IndexMap<String, Data>,
		format: Option<VersionDependent<Vec<state_spec::FieldFormat>>>,
	) -> Self {
		let mut fields = Vec::new();
		for (field_name, field) in data {
			let datatype;
			let feature;

			match field {
				Data::Normal(t) => {
					datatype = t.as_tokenstream();
					feature = None;
				}
				Data::RequiresFeature {
					feature: f,
					data_type: t,
					default,
				} => {
					datatype = t.as_tokenstream();

					feature = Some(FieldFeatureReq {
						feature: Feature { feature: f },
						default_value: default.as_tokenstream(),
					});
				}
			}

			fields.push(Field {
				name: field_name.as_ident(),
				datatype,
				feature,
			});
		}

		let format = match format {
			Some(format) => format
				.expand_shortcut()
				.into_iter()
				.map(|(bounds, format)| {
					(
						bounds,
						format.into_iter().map(FieldFormat::from_spec).collect(),
					)
				})
				.collect(),
			None => IndexMap::from([(
				vec![Bounds::All],
				fields
					.iter()
					.map(|f| FieldFormat {
						field: Some(f.name.clone()),
						format: CustomFormat::default(),
					})
					.collect(),
			)]),
		};

		FieldsContainer { fields, format }
	}

	/// Generates `pub field: Type, ...`
	pub fn gen_definition(&self) -> TokenStream {
		if self.fields.is_empty() {
			return quote! {};
		}

		let fields = self.fields.iter().map(|f| {
			f.gen(FieldGenOptions {
				with_type: true,
				with_pub: true,
				with_feature_cfg: true,
				with_default_value: false,
				with_feature_doc_note: true,
			})
		});

		quote! {
			#( #fields, )*
		}
	}
	/// works both for constructing and deconstructing field containers
	/// Generates `field, ... `
	pub fn gen_constructor(&self) -> TokenStream {
		if self.fields.is_empty() {
			return quote! {};
		}

		let fields = self.fields.iter().map(|f| {
			f.gen(FieldGenOptions {
				with_type: false,
				with_pub: false,
				with_feature_cfg: true,
				with_default_value: false,
				with_feature_doc_note: false,
			})
		});

		quote! {
			#( #fields, )*
		}
	}
	/// Generates code that reads all fields into their respective variables
	/// In a MinecraftProtocol implementation
	pub fn gen_read_impl(&self) -> TokenStream {
		let mut result = TokenStream::new();

		// First we create a variable for each field that is about to be read,
		// automatically initializing those fields who depend on a feature with their
		// default value, in case we're dealing with a client with a protocol version
		// that doesn't support that feature.
		for field in &self.fields {
			let field_name = &field.name;

			let default = field.feature.as_ref().map(|f| &f.default_value).into_iter();

			result.extend(quote! {
				#[allow(unused_variables, unused_assignments, unused_mut)]
				let mut #field_name #( = #default)*;
			});
		}

		// Now match the protocol version and read the fields according to the format
		let mut arms = Vec::new();
		for (bounds, format) in &self.format {
			let pattern = bounds.as_match_pattern();

			let mut field_reads = Vec::new();
			for f in format {
				let read = match (&f.format.custom_read, f.field.is_some()) {
					(None, true) => {
						// default field read, type will be inferred from the field variable
						quote! {
							crate::MinecraftProtocol::read(___PROTOCOL_VERSION___, ___INPUT___)?
						}
					}
					(None, false) => {
						// No field, no custom read, just skip this
						continue;
					}
					(Some(custom_read), _) => {
						let data_type = &custom_read.read_as;
						let read = &custom_read.read;
						quote! {
							{
								#[allow(non_snake_case, unused_variables)]
								let THIS: #data_type = crate::MinecraftProtocol::read(___PROTOCOL_VERSION___, ___INPUT___)?;
								#read
							}
						}
					}
				};

				field_reads.push(match &f.field {
					Some(field_name) => {
						// if this field requires a feature, add a cfg to allow unused_assignments when that feature is not enabled
						let cfg_attr = self
							.fields
							.iter()
							.find(|f| &f.name == field_name)
							.expect("custom format field doesnt exist")
							.feature
							.as_ref()
							.map(|f| {
								let feature_name = &f.feature.feature;
								quote! { #[cfg_attr(not(feature = #feature_name), allow(unused_assignments))] }
							});

						quote! { #cfg_attr { #field_name = #read; } }
					}
					None => quote! { #read; },
				});
			}

			arms.push(quote! {
				#pattern => {
					#( #field_reads )*
				},
			});
		}

		result.extend(quote! {
			match ___PROTOCOL_VERSION___ {
				#( #arms )*
			}
		});

		result
	}
	/// Generates code that writes all fields to the output
	/// In a MinecraftProtocol implementation
	///
	/// all fields must already be prepared as normal variables
	/// destructure your structure before generating this
	pub fn gen_write_impl(&self) -> TokenStream {
		let mut result = TokenStream::new();

		// Now match the protocol version and write the fields according to the format
		let mut arms = Vec::new();
		for (bounds, format) in &self.format {
			let pattern = bounds.as_match_pattern();

			let mut field_writes = Vec::new();
			for f in format {
				let write = match (&f.format.custom_write, &f.field) {
					(None, Some(field_name)) => {
						// default field write
						quote! {
							___WRITTEN_BYTES___ += crate::MinecraftProtocol::write(
								#field_name,
								___PROTOCOL_VERSION___,
								___OUTPUT___
							)?;
						}
					}
					(None, None) => {
						// No field, no custom write, just skip this
						continue;
					}
					(Some(custom_write), Some(field_name)) => {
						quote! {
							___WRITTEN_BYTES___ += {
								#[allow(non_snake_case, unused_variables)]
								let THIS = &#field_name;
								crate::MinecraftProtocol::write({ #custom_write }.borrow(), ___PROTOCOL_VERSION___, ___OUTPUT___)?
							};
						}
					}
					(Some(custom_write), None) => {
						quote! {
							___WRITTEN_BYTES___ += crate::MinecraftProtocol::write(
								{ #custom_write }.borrow(),
								___PROTOCOL_VERSION___,
								___OUTPUT___
							)?;
						}
					}
				};

				field_writes.push(quote! { #write });
			}

			arms.push(quote! {
				#pattern => {
					#( #field_writes )*
				},
			});
		}

		result.extend(quote! {
			match ___PROTOCOL_VERSION___ {
				#( #arms )*
			}
		});

		result
	}
}
