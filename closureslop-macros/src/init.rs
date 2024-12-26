use crate::collector_name;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
	LitStr, Path, Token,
	parse::{Parse, ParseStream, Result},
	parse_macro_input,
};

struct Args {
	closureslop_crate: Option<Path>,
	context_path: Path,
	id: Option<LitStr>,
}

impl Parse for Args {
	fn parse(input: ParseStream) -> Result<Self> {
		let closureslop_crate: Option<Path> = if input.peek(Token![@]) {
			input.parse::<Token![@]>()?;
			Some(input.parse()?)
		} else {
			None
		};

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

		Ok(Self {
			closureslop_crate,
			context_path,
			id,
		})
	}
}

pub fn init(args: TokenStream) -> TokenStream {
	let Args {
		closureslop_crate,
		context_path,
		id,
	} = parse_macro_input!(args as Args);

	let closureslop_path = match closureslop_crate {
		Some(path) => quote! { #path },
		None => quote!(::closureslop),
	};

	let static_name = collector_name(&id);

	quote! {
		#[doc(hidden)]
		#[allow(non_upper_case_globals)]
		#[#closureslop_path::__private_macroslop::linkme::distributed_slice]
		#[linkme(crate = #closureslop_path::__private_macroslop::linkme)]
		pub static #static_name: [fn(&mut #closureslop_path::Reactor<#context_path>)];
	}
	.into()
}
