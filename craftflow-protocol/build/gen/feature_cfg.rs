use proc_macro2::TokenStream;
use quote::quote;

// Generates:
//
// #[cfg(any(feature = "my_feature", doc))]
// #[doc(cfg(feature = "my_feature"))]
pub fn gen_feature_cfg(feature: &Option<impl AsRef<str>>) -> TokenStream {
	match feature {
		Some(feature) => {
			let feature = feature.as_ref();
			quote! {
				#[cfg(any(feature = #feature, doc))]
				#[doc(cfg(feature = #feature))]
			}
		}
		None => quote! {},
	}
}
