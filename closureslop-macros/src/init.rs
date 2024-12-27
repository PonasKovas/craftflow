use crate::collector_name;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
	Ident, LitStr, Path, Token, Type,
	parse::{Parse, ParseStream, Result},
	parse_macro_input,
};

struct Args {
	closureslop_crate: Option<Path>,
	id: Option<LitStr>,
	context: Type,
}

impl Parse for Args {
	fn parse(input: ParseStream) -> Result<Self> {
		let closureslop_crate: Option<Path> = if input.peek(Token![@]) {
			input.parse::<Token![@]>()?;
			Some(input.parse()?)
		} else {
			None
		};

		let mut id = None;
		let mut context = None;
		loop {
			let keyword = match input.parse::<Ident>() {
				Ok(keyword) => keyword,
				Err(_) => break,
			};
			input.parse::<Token![:]>()?;

			match keyword.to_string().as_str() {
				"group" => {
					if id.is_some() {
						return Err(input.error("unexpected group"));
					}
					id = Some(input.parse()?);
				}
				"ctx" => {
					if context.is_some() {
						return Err(input.error("unexpected ctx"));
					}
					context = Some(input.parse()?);
				}
				_ => {
					return Err(input.error("unexpected keyword"));
				}
			}

			if input.peek(Token![,]) {
				input.parse::<Token![,]>()?;
			}
		}

		let context = match context {
			Some(ctx) => ctx,
			None => return Err(input.error("missing ctx")),
		};

		Ok(Self {
			closureslop_crate,
			context,
			id,
		})
	}
}

pub fn init(args: TokenStream) -> TokenStream {
	let Args {
		closureslop_crate,
		context,
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
		pub static #static_name: [fn(&mut #closureslop_path::Reactor<#context>)];
	}
	.into()
}
