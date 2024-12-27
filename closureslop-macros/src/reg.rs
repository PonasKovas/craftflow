use crate::collector_name;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
	Expr, Ident, LitStr, Token,
	parse::{Parse, ParseStream, Result},
	parse_macro_input,
};

struct Args {
	id: Option<LitStr>,
	reactor: Expr,
}

impl Parse for Args {
	fn parse(input: ParseStream) -> Result<Self> {
		let mut id = None;
		let mut reactor = None;
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
				"to" => {
					if reactor.is_some() {
						return Err(input.error("unexpected to"));
					}
					reactor = Some(input.parse()?);
				}
				_ => {
					return Err(input.error("unexpected keyword"));
				}
			}

			if input.peek(Token![,]) {
				input.parse::<Token![,]>()?;
			}
		}

		let reactor = match reactor {
			Some(r) => r,
			None => return Err(input.error("missing reactor")),
		};

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
