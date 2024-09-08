use super::{custom_format::CustomFormat, feature::Feature};
use crate::build::{gen::feature::FeatureCfgOptions, state_spec, util::AsIdent};
use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub struct Field {
	pub name: Ident,
	pub datatype: TokenStream,
	pub feature: Option<FieldFeatureReq>,
}

pub struct FieldFeatureReq {
	pub feature: Feature,
	pub default_value: TokenStream,
}

pub struct FieldFormat {
	/// If the field is None, the value read will not be assigned to anything
	pub field: Option<Ident>,
	pub format: CustomFormat,
}

#[derive(Clone, Copy)]
pub struct FieldGenOptions {
	/// [field]: Type
	pub with_type: bool,
	/// pub [field]
	pub with_pub: bool,
	/// #[cfg(feature = "my_feature")] [field]
	pub with_feature_cfg: bool,
	/// [field] = default
	pub with_default_value: bool,
	pub with_feature_doc_note: bool,
}

impl Field {
	pub fn gen(&self, options: FieldGenOptions) -> TokenStream {
		let pub_keyword = options.with_pub.then(|| quote! { pub });

		let field_name = &self.name;

		let field_type = options.with_type.then(|| &self.datatype).into_iter();

		let feature_cfg;
		let default;
		match &self.feature {
			None => {
				default = None.into_iter();
				feature_cfg = None;
			}
			Some(feature) => {
				default = options
					.with_default_value
					.then(|| &feature.default_value)
					.into_iter();
				feature_cfg = options.with_feature_cfg.then(|| {
					feature.feature.gen_cfg(FeatureCfgOptions {
						negative: false,
						with_doc_note: options.with_feature_doc_note,
					})
				});
			}
		}

		quote! {
			#feature_cfg
			#pub_keyword #field_name #( : #field_type )* #( = #default )*
		}
	}
}

impl FieldFormat {
	pub fn from_spec(spec: state_spec::FieldFormat) -> Self {
		FieldFormat {
			field: spec.field.as_ref().map(|f| f.as_ident()),
			format: CustomFormat::from_field_format(&spec),
		}
	}
}
