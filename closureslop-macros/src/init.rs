use crate::collector_name;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
	LitStr, Path, Token,
	parse::{Parse, ParseStream, Result},
	parse_macro_input,
};

struct Args {
	context_path: Path,
	id: Option<LitStr>,
}

impl Parse for Args {
	fn parse(input: ParseStream) -> Result<Self> {
		let context_path: Path = input.parse()?;

		let id: Option<LitStr> = if input.peek(Token![,]) {
			input.parse::<Token![,]>()?;
			Some(input.parse()?)
		} else {
			None
		};

		// Allow a trailing comma at the end
		if input.peek(Token![,]) {
			input.parse::<Token![,]>()?;
		}

		Ok(Self { context_path, id })
	}
}

pub fn init(args: TokenStream) -> TokenStream {
	let Args { context_path, id } = parse_macro_input!(args as Args);

	let static_name = collector_name(&id);

	quote! {
		#[doc(hidden)]
		#[allow(non_upper_case_globals)]
		#[::closureslop::__private_macroslop::linkme::distributed_slice]
		#[linkme(crate = ::closureslop::__private_macroslop::linkme)]
		pub static #static_name: [fn(&mut ::closureslop::Reactor<#context_path>)];
	}
	.into()
}
