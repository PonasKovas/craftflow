use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{Ident, LitStr};

mod callback;
mod init;
mod reg;

#[proc_macro]
pub fn init(args: TokenStream) -> TokenStream {
	init::init(args)
}

#[proc_macro]
pub fn reg(args: TokenStream) -> TokenStream {
	reg::reg(args)
}

#[proc_macro_attribute]
pub fn callback(args: TokenStream, input: TokenStream) -> TokenStream {
	callback::callback(args, input)
}

fn collector_name(id: &Option<LitStr>) -> Ident {
	let suffix = match id {
		Some(id) => &id.value(),
		None => "CALLBACKS",
	};

	// we need to get an unique identifier for the specific crate using this macro, so their collectors dont conflict
	let crate_id = crate_id();

	Ident::new(
		&format!("__PRIVATE_CLOSURESLOP_{crate_id}_{suffix}"),
		Span::call_site(),
	)
}

/// Returns an unique identifier for this specific crate being compiled
fn crate_id() -> String {
	// these three env vars should never be the same in different compilation objects in the dependency tree
	let crate_name = std::env::var("CARGO_CRATE_NAME").unwrap();
	let pkg_name = std::env::var("CARGO_PKG_NAME").unwrap();
	let version = std::env::var("CARGO_PKG_VERSION").unwrap();

	format!("{crate_name}_{}", fnv1a_hash(&(pkg_name + &version)))
}

fn fnv1a_hash(input: &str) -> u64 {
	let mut hash: u64 = 14695981039346656037; // FNV offset basis

	for byte in input.as_bytes() {
		hash ^= *byte as u64;
		hash = hash.wrapping_mul(1099511628211); // FNV prime
	}

	hash
}
