use super::gen::feature_cfg::gen_feature_cfg;
use super::version_bounds::parse_bounds;
use super::version_bounds::Bounds;
use proc_macro2::TokenStream;
use quote::quote;
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs;

/// Parsed protocol.ron
#[derive(Deserialize, Debug)]
pub struct Info {
	/// All supported protocol versions
	pub supported_protocols: Vec<Bounds>,
	/// All protocol features and what protocol versions support them
	pub features: BTreeMap<String, Vec<Bounds>>,
}

pub fn parse_info_file(path: &str) -> Result<Info, Box<dyn Error>> {
	let file_contents = fs::read_to_string(path)?;
	let state_spec: Info = ron::from_str(&file_contents)?;

	Ok(state_spec)
}

impl Info {
	/// Calculates all unsupported protocol versions with the current feature configuration
	/// And formats them as a bunch of patterns for a match statement with the given tokenstream
	// on the right size of each pattern
	pub fn unsupported_versions_patterns(&self, right_side: TokenStream) -> TokenStream {
		let all_supported = self.all_supported();
		let minimum = all_supported.iter().min().unwrap() - 1;
		let maximum = all_supported.iter().max().unwrap() + 1;

		let mut result = quote! {
			// two arms that match the whole -INF... ...INF versions
			#[allow(unreachable_patterns)]
			..=#minimum => #right_side,
			#[allow(unreachable_patterns)]
			#maximum.. => #right_side,
		};

		for version in all_supported {
			let mut features = Vec::new();

			for (feature, bounds) in &self.features {
				// if the bounds dont contain this version
				// that means this version is not supported if the feature is enabled
				if !Bounds::contain(bounds, version) {
					features.push(quote! { #feature });
				}
			}

			result.extend(quote! {
				#[cfg(any( #( feature = #features ),* ))]
				#[allow(unreachable_patterns)]
				#version => #right_side,
			});
		}

		result
	}
	/// Returns a list of all supported versions (not taking features into account)
	fn all_supported(&self) -> Vec<u32> {
		let mut result = BTreeSet::new();
		for bounds in &self.supported_protocols {
			match *bounds {
				Bounds::All | Bounds::From(_) | Bounds::UpTo(_) => {
					panic!(
						"supported protocol versions must be finite. *, + and - are not allowed."
					);
				}
				Bounds::Range(start, end) => {
					for i in start..=end {
						result.insert(i);
					}
				}
				Bounds::Concrete(version) => {
					result.insert(version);
				}
			}
		}

		result.into_iter().collect()
	}
}
