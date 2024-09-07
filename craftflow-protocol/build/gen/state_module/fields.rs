use crate::build::{
	gen::feature_cfg::{gen_feature_cfg, gen_not_feature_cfg},
	state_spec::{Data, FieldFormat, VersionDependent},
	version_bounds::Bounds,
	AsIdent, AsTokenStream, Info,
};
use indexmap::IndexMap;
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Copy, Clone)]
pub struct Fields<'a> {
	pub data: &'a IndexMap<String, Data>,
	pub format: &'a Option<VersionDependent<Vec<FieldFormat>>>,
}

impl<'a> Fields<'a> {
	// Generates:
	//
	// [ #[cfg(feature = "feature")] ]
	// [pub] my_field [: String]
	// [pub] another [: u32]
	// ...
	pub fn gen(
		self,
		with_types: bool,
		with_pub: bool,
		with_feature_cfg: bool,
		with_defaults: bool,
	) -> Vec<TokenStream> {
		let mut result = Vec::new();

		let opt_pub = with_pub.then(|| quote! { pub });

		for (field_name, field) in self.data {
			let field_name = field_name.as_ident();

			let field_type = with_types.then(|| {
				let t = field.datatype();
				quote! { : #t }
			});

			let default;
			let feature_cfg;
			match field {
				Data::Normal(_) => {
					default = None;
					feature_cfg = None;
				}
				Data::RequiresFeature {
					feature,
					data_type: _,
					default: def,
				} => {
					default = with_defaults.then(|| {
						let d = def.as_tokenstream();
						quote! { = #d }
					});
					feature_cfg = with_feature_cfg.then(|| gen_feature_cfg(feature, true));
				}
			}

			result.push(quote! {
				#feature_cfg
				#opt_pub #field_name #field_type #default
			});
		}

		result
	}

	// Generates:
	//
	// field_name = MinecraftProtocol::read(___PV___, ___S___)?;
	// another = MinecraftProtocol::read(___PV___, ___S___)?;
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
									#field_name = crate::MinecraftProtocol::read(___PROTOCOL_VERSION___, ___INPUT___)?;
								}
							}
						}
						Data::RequiresFeature {
							feature,
							data_type: _,
							default: _,
						} => {
							// Only read the field if the protocol version matches the feature
							// otherwise leave default
							let feature_bounds = &info.features[feature];
							let protocol_pattern = Bounds::as_match_pattern(feature_bounds);

							quote! {
								if let #protocol_pattern = ___PROTOCOL_VERSION___ {
									#[allow(unused_assignments)] {
										#field_name = crate::MinecraftProtocol::read(___PROTOCOL_VERSION___, ___INPUT___)?;
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

					let mut reads = Vec::new();
					for field in format {
						let field_name = field.field.as_ident();

						match (field.read_as, field.read) {
							(None, Some(_)) | (Some(_), None) => panic!(
								"read and read_as have to come together. must not use them alone"
							),
							(None, None) => {
								// Normal read
								reads.push(quote! {
									#[allow(unused_assignments)] {
										#field_name = crate::MinecraftProtocol::read(___PROTOCOL_VERSION___, ___INPUT___)?;
									}
								});
							}
							(Some(read_as), Some(read)) => {
								// Special read
								let read_as = read_as.as_tokenstream();
								let read = read.as_tokenstream();

								reads.push(quote! {
									#[allow(unused_assignments)] {
										#field_name = {
											let #field_name: #read_as = crate::MinecraftProtocol::read(___PROTOCOL_VERSION___, ___INPUT___)?;
											#read
										};
									}
								});
							}
						}
					}

					match_arms.push(quote! {
						#protocol_pattern => {
							#( #reads )*
						},
					});
				}

				let unsupported_versions_arms = info.unsupported_versions_patterns();

				result = quote! {
					match ___PROTOCOL_VERSION___ {
						#( #match_arms )*

						// Match all versions that are not supported with the
						// current feature set so that we would get a compile error if
						// there are any possible uncovered protocol versions
						#( #unsupported_versions_arms => unreachable!("this protocol version is not supported with the current enabled feature set"), )*
					}
				};
			}
		}

		result
	}

	/// Generates:
	///
	/// ___WRITTEN_BYTES___ += MinecraftProtocol::write(my_field, ___PROTOCOL_VERSION___, ___OUTPUT___)?;
	/// ___WRITTEN_BYTES___ += MinecraftProtocol::write(another, ___PROTOCOL_VERSION___, ___OUTPUT___)?;
	pub fn gen_minecraftprotocol_write(self, info: &Info) -> TokenStream {
		let mut result = TokenStream::new();

		match self.format {
			None => {
				// No special format so just default by fields
				for (field_name, field) in self.data {
					let field_name = field_name.as_ident();

					result.extend(match field {
						Data::Normal(_data_type) => {
							// Just write the field normally

							quote! {
								___WRITTEN_BYTES___ += crate::MinecraftProtocol::write(#field_name, ___PROTOCOL_VERSION___, ___OUTPUT___)?;
							}
						}
						Data::RequiresFeature {
							feature,
							data_type: _,
							default: _,
						} => {
							let feature_bounds = &info.features[feature];
							let protocol_pattern = Bounds::as_match_pattern(feature_bounds);

							quote! {
								if let #protocol_pattern = ___PROTOCOL_VERSION___ {
									___WRITTEN_BYTES___ += crate::MinecraftProtocol::write(#field_name, ___PROTOCOL_VERSION___, ___OUTPUT___)?;
								};
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

					let mut writes = Vec::new();
					for field in format {
						let field_name = field.field.as_ident();

						match &field.write {
							None => {
								// Normal write
								writes.push(quote! {
									___WRITTEN_BYTES___ += crate::MinecraftProtocol::write(#field_name, ___PROTOCOL_VERSION___, ___OUTPUT___)?;
								});
							}
							Some(write) => {
								// Special write
								let write = write.as_tokenstream();

								writes.push(quote! {
									___WRITTEN_BYTES___ += crate::MinecraftProtocol::write(
										{
											#write
										}.borrow(),
										___PROTOCOL_VERSION___,
										___OUTPUT___
									)?;
								});
							}
						}
					}

					match_arms.push(quote! {
						#protocol_pattern => {
							#( #writes )*
						},
					});
				}

				let unsupported_versions_arms = info.unsupported_versions_patterns();

				result = quote! {
					match ___PROTOCOL_VERSION___ {
						#( #match_arms )*

						// Match all versions that are not supported with the
						// current feature set so that we would get a compile error if
						// there are any possible uncovered protocol versions
						#( #unsupported_versions_arms => unreachable!("this protocol version is not supported with the current enabled feature set"), )*
					}
				};
			}
		}

		result
	}
}
