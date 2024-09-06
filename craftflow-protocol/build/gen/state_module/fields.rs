use crate::build::{
	gen::feature_cfg::gen_feature_cfg,
	state_spec::{Data, FieldFormat, VersionDependent},
	version_bounds::Bounds,
	AsIdent, AsTokenStream, Info,
};
use indexmap::IndexMap;
use proc_macro2::TokenStream;
use quote::quote;

pub struct Fields<'a> {
	pub data: &'a IndexMap<String, Data>,
	pub format: &'a Option<VersionDependent<Vec<FieldFormat>>>,
}

impl<'a> Fields<'a> {
	// Generates:
	//
	// my_field: String
	// another: u32
	// ...
	pub fn gen_defs(
		self,
		with_types: bool,
		with_pub: bool,
		with_feature_cfg: bool,
	) -> Vec<TokenStream> {
		let mut result = Vec::new();

		let opt_pub = if with_pub {
			quote! { pub }
		} else {
			quote! {}
		};

		for (field_name, field) in self.data {
			let field_name = field_name.as_ident();

			let feature_dependency;
			let field_type;

			match field {
				Data::Normal(data_type) => {
					feature_dependency = None;
					field_type = data_type.as_ident();
				}
				Data::RequiresFeature {
					feature,
					data_type,
					default: _,
				} => {
					feature_dependency = Some(feature);
					field_type = data_type.as_ident();
				}
			};

			let feature_cfg = if with_feature_cfg {
				gen_feature_cfg(&feature_dependency)
			} else {
				quote! {}
			};

			let field_type = if with_types {
				quote! { : #field_type }
			} else {
				quote! {}
			};

			result.push(quote! {
				#feature_cfg
				#opt_pub #field_name #field_type
			});
		}

		result
	}

	// Generates:
	//
	// field_name = FieldType::read(___PV___, ___S___)?;
	// another = u32::read(___PV___, ___S___)?;
	// ..
	pub fn gen_minecraftprotocol_read(self, info: &Info) -> TokenStream {
		let mut result = TokenStream::new();

		match self.format {
			None => {
				// No special format so just default by fields
				for (field_name, field) in self.data {
					let field_name = field_name.as_ident();

					result.extend(match field {
						Data::Normal(_data_type) => {
							// Just read the field normally

							quote! {
								#[allow(unused_assignments)] {
									#field_name = crate::MinecraftProtocol::read(___PROTOCOL_VERSION___, ___SOURCE___)?;
								}
							}
						}
						Data::RequiresFeature {
							feature,
							data_type: _,
							default: _,
						} => {
							// Only read the field if the protocol version matches the feature
							// otherwise give default
							let feature_bounds = &info.features[feature];
							let protocol_pattern = Bounds::as_match_pattern(feature_bounds);

							quote! {
								if let #protocol_pattern = ___PROTOCOL_VERSION___ {
									#[allow(unused_assignments)] {
										#field_name = crate::MinecraftProtocol::read(___PROTOCOL_VERSION___, ___SOURCE___)?;
									}
								}
							}
						}
					});
				}
			}
			Some(format) => {
				// Special format depending on protocol version
				let mut match_arms = Vec::new();
				for (bounds, format) in format.expand_shortcut() {
					let protocol_pattern = Bounds::as_match_pattern(&bounds);

					let mut lines = Vec::new();
					for field in format {
						let field_name = field.field.as_ident();

						match (field.read_as, field.read) {
							(None, Some(_)) | (Some(_), None) => panic!(
								"read and read_as have to come together. must not use them alone"
							),
							(None, None) => {
								lines.push(quote! {
									#[allow(unused_assignments)] {
										#field_name = crate::MinecraftProtocol::read(___PROTOCOL_VERSION___, ___SOURCE___)?;
									}
								});
							}
							(Some(read_as), Some(read)) => {
								let read_as = read_as.as_tokenstream();
								let read = read.as_tokenstream();

								lines.push(quote! {
									#[allow(unused_assignments)] {
										#field_name = {
											let #field_name: #read_as = crate::MinecraftProtocol::read(___PROTOCOL_VERSION___, ___SOURCE___)?;
											#read
										};
									}
								});
							}
						}
					}

					match_arms.push(quote! {
						#protocol_pattern => {
							#( #lines )*
						},
					});
				}

				let unsupported_versions_arms = info.unsupported_versions_patterns(quote! {
					unreachable!("this protocol version is not supported with the current enabled feature set")
				});

				result = quote! {
					match ___PROTOCOL_VERSION___ {
						#( #match_arms )*

						// Match all versions that are not supported with the
						// current feature set so that we would get a compile error if
						// there are any possible uncovered versions
						#unsupported_versions_arms
					}
				};
			}
		}

		result
	}
}
