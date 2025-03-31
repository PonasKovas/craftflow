//! This crate just wraps the `closureslop-macros` proc macros and gives them the re-exported path to `closureslop`
//! crate so they can work without the user importing `closurelop` crate directly.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

#[proc_macro]
pub fn init(args: TokenStream) -> TokenStream {
	let args = TokenStream2::from(args);

	quote! {
		::craftflow::closureslop::init!(@::craftflow::closureslop ctx: std::sync::Arc<::craftflow::CraftFlow>, #args);
	}
	.into()
}

#[proc_macro]
pub fn reg(args: TokenStream) -> TokenStream {
	let args = TokenStream2::from(args);

	quote! {
		::craftflow::closureslop::reg!(#args);
	}
	.into()
}

#[proc_macro_attribute]
pub fn callback(args: TokenStream, input: TokenStream) -> TokenStream {
	let args = TokenStream2::from(args);
	let input = TokenStream2::from(input);

	quote! {
		#[::craftflow::closureslop::callback(@::craftflow::closureslop #args)]
		#input
	}
	.into()
}
