use proc_macro2::TokenStream;
use quote::quote;

// Generates:
//
// #[cfg(any(feature = "my_feature", doc))]
// #[doc(cfg(feature = "my_feature"))]
pub fn gen_feature_cfg(feature: impl AsRef<str>, with_doc: bool) -> TokenStream {
	let feature = feature.as_ref();

	if with_doc {
		quote! {
			#[cfg(any(feature = #feature, doc))]
			#[doc(cfg(feature = #feature))]
		}
	} else {
		quote! {
			#[cfg(feature = #feature)]
		}
	}
}

// Generates:
//
// #[cfg(any(not(feature = "my_feature"), doc))]
// #[doc(cfg(not(feature = "my_feature")))]
pub fn gen_not_feature_cfg(feature: impl AsRef<str>, with_doc: bool) -> TokenStream {
	let feature = feature.as_ref();

	if with_doc {
		quote! {
			#[cfg(any(not(feature = #feature), doc))]
			#[doc(cfg(not(feature = #feature)))]
		}
	} else {
		quote! {
			#[cfg(not(feature = #feature))]
		}
	}
}
