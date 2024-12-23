use crate::collector_name;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{
	ItemFn, LitStr, Path, Token, parenthesized,
	parse::{Parse, ParseStream, Result},
	parse_macro_input,
};

struct Args {
	id: Option<LitStr>,
	event: Path,
	order_info: TokenStream2,
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

		let event: Path = input.parse()?;

		// Allow a trailing comma at the end
		if input.peek(Token![,]) {
			input.parse::<Token![,]>()?;
		}

		// if there is anything more, it must be the order info
		// which will be handled by the inner (declarative) macro
		// (located in closureslop crate)
		let order_info = input.parse()?;

		Ok(Self {
			id,
			event,
			order_info,
		})
	}
}

pub fn callback(args: TokenStream, input: TokenStream) -> TokenStream {
	let Args {
		id,
		event,
		order_info,
	} = parse_macro_input!(args as Args);
	let input = parse_macro_input!(input as ItemFn);

	let function_name = &input.sig.ident;
	let callback_name = LitStr::new(&input.sig.ident.to_string(), Span::call_site());

	// name of the linkme collector thats expected to be found at the root of the crate
	let collector_name = collector_name(&id);

	// also we need to get the type of the first argument, which is the the context
	// we need it to write out the type of the reactor
	let context_path = get_context_type(&input);

	let argument_count = input.sig.inputs.len() - 1; // excluding the context argument
	let args: Vec<_> = (0..argument_count)
		.map(|i| {
			let index = syn::Index::from(i); // Generate index for arg.i
			quote! { args.#index }
		})
		.collect();

	quote! {
		const _: () = {
			#[::closureslop::__private_macroslop::distributed_slice(crate::#collector_name)]
			fn _add_callback(reactor: &mut ::closureslop::Reactor<#context_path>) {
				let wrapper = |ctx, mut args| {
					// clever hack to make sure we have the right number of arguments
					// (this will fail to compile if the user given callback function takes less arguments than expected)
					args = (#(#args),*);

					::closureslop::__private_macroslop::SmallBox::new(async move {
						#function_name(ctx, #(#args),*).await
					})
				};
				::closureslop::add_callback!(reactor, #event => #callback_name => wrapper, #order_info);
			}
		};

		#input
	}
	.into()
}

fn get_context_type(input: &ItemFn) -> syn::Type {
	let arg = input
		.sig
		.inputs
		.iter()
		.next()
		.expect("callback function must have at least 1 argument");

	let arg = match arg {
		syn::FnArg::Receiver(_) => {
			panic!("callback function cannot use self argument");
		}
		syn::FnArg::Typed(pat_type) => pat_type,
	};

	(*arg.ty).clone()
}
