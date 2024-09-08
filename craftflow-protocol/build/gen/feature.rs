use proc_macro2::TokenStream;
use quote::quote;

#[derive(Clone)]
pub struct Feature {
	pub feature: String,
}

#[derive(Clone, Copy)]
pub struct FeatureCfgOptions {
	pub negative: bool,
	pub with_doc_note: bool,
}

impl Feature {
	pub fn gen_cfg(&self, options: FeatureCfgOptions) -> TokenStream {
		let feature_name = &self.feature;

		let inner = match options.negative {
			false => quote! { feature = #feature_name },
			true => quote! { not(feature = #feature_name) },
		};

		match options.with_doc_note {
			true => quote! { #[cfg(any(#inner, doc))] #[doc(cfg(#inner))] },
			false => quote! { #[cfg(#inner)] },
		}
	}
}
