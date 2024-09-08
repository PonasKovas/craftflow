use crate::build::{
	info_file::Info,
	version_bounds::{Bounds, BoundsMethods},
};
use indexmap::IndexMap;
use proc_macro2::TokenStream;
use quote::quote;

pub struct VersionDependent<T> {
	pub inner: IndexMap<Vec<Bounds>, T>,
}

impl<T> VersionDependent<T> {
	/// Generates a match statement that matches the protocol version, generating the right side
	/// of the arms with the given closure
	/// Also adds match arms for all unsupported protocol versions so we would get a compile
	/// error if any versions are not covered by this
	pub fn gen_protocol_version_match(
		&self,
		info: &Info,
		mut right_side: impl FnMut(&T) -> TokenStream,
	) -> TokenStream {
		let unsupported_versions_arms = info.unsupported_versions_patterns();

		let mut arms = Vec::new();
		for (bounds, value) in &self.inner {
			let pattern = bounds.as_match_pattern();

			let right_side = right_side(value);

			arms.push(quote! {
				#pattern => { #right_side },
			});
		}

		quote! {
			match ___PROTOCOL_VERSION___ {
				#( #arms )*

				// Match all versions that are not supported with the
				// current feature set so that we would get a compile error if
				// there are any possible uncovered protocol versions
				#( #unsupported_versions_arms => unreachable!(), )*
			}
		}
	}
}

impl<T> From<IndexMap<Vec<Bounds>, T>> for VersionDependent<T> {
	fn from(value: IndexMap<Vec<Bounds>, T>) -> Self {
		VersionDependent { inner: value }
	}
}
