use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{Ident, LitStr};

mod callback;
mod init;

#[proc_macro]
pub fn init(args: TokenStream) -> TokenStream {
	init::init(args)
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

	Ident::new(
		&format!("__PRIVATE_CLOSURESLOP_{suffix}"),
		Span::call_site(),
	)
}
