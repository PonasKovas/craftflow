use crate::collector_name;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
	Expr, LitStr, Token, parenthesized,
	parse::{Parse, ParseStream, Result},
	parse_macro_input,
};

struct Args {
	id: Option<LitStr>,
	reactor: Expr,
}

impl Parse for Args {
	fn parse(input: ParseStream) -> Result<Self> {
		let id = if input.peek(syn::token::Paren) {
			let content;
			parenthesized!(content in input);

			input.parse::<Token![,]>()?;

			Some(content.parse()?)
		} else {
			None
		};

		let reactor: Expr = input.parse()?;

		// Allow a trailing comma at the end
		if input.peek(Token![,]) {
			input.parse::<Token![,]>()?;
		}

		Ok(Self { id, reactor })
	}
}

pub fn reg(args: TokenStream) -> TokenStream {
	let Args { id, reactor } = parse_macro_input!(args as Args);

	let static_name = collector_name(&id);

	quote! {
		{
			let reactor = #reactor;
			for f in &*crate::#static_name {
				f(reactor);
			}
		}
	}
	.into()
}
