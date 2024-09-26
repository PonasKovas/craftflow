use proc_macro2::TokenStream;
use quote::quote;
use serde_json::Value;
use std::collections::HashMap;

use crate::VERSIONS;

pub fn generate(versions: HashMap<u32, Value>) -> TokenStream {
	// iterate over all supported versions in ascending order
	let mut supported_versions = VERSIONS.to_owned();
	supported_versions.sort();
	let mut last_defined_version = None;

	for version in supported_versions {
		match versions.get(&version) {
			Some(protocol) => {
				// this version has a spec, that means it's different from the previous one
				last_defined_version = Some(version);
			}
			None => {
				// this version has no spec, that means it's the same as the previous one
			}
		}
	}
	quote! {}
}
