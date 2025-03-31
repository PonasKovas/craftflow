use crate::collector_name;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
	Error, Ident, ItemFn, LitStr, Path, Token, Type,
	parse::{Parse, ParseStream, Result},
	parse_macro_input,
};

struct Args {
	closureslop_crate: Option<Path>,
	id: Option<LitStr>,
	event: Type,
	order: OrderInfo,
}

struct OrderInfo {
	before: Vec<LitStr>,
	after: Vec<LitStr>,
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
		let mut event = None;
		let mut before = Vec::new();
		let mut after = Vec::new();
		while let Ok(keyword) = input.parse::<Ident>() {
			input.parse::<Token![:]>()?;

			match keyword.to_string().as_str() {
				"group" => {
					if id.is_some() {
						return Err(Error::new(keyword.span(), "unexpected group"));
					}
					id = Some(input.parse()?);
				}
				"event" => {
					if event.is_some() {
						return Err(Error::new(keyword.span(), "unexpected event"));
					}
					event = Some(input.parse()?);
				}
				"before" => {
					before.push(input.parse()?);
				}
				"after" => {
					after.push(input.parse()?);
				}
				_ => {
					return Err(input.error("unexpected keyword"));
				}
			}

			if input.peek(Token![,]) {
				input.parse::<Token![,]>()?;
			}
		}

		let event = match event {
			Some(ev) => ev,
			None => return Err(Error::new(input.span(), "missing event")),
		};

		Ok(Self {
			closureslop_crate,
			id,
			event,
			order: OrderInfo { before, after },
		})
	}
}

pub fn callback(args: TokenStream, input: TokenStream) -> TokenStream {
	let Args {
		closureslop_crate,
		id,
		event,
		order: OrderInfo { before, after },
	} = parse_macro_input!(args as Args);
	let input = parse_macro_input!(input as ItemFn);

	let closureslop_path = match closureslop_crate {
		Some(path) => quote! { #path },
		None => quote!(::closureslop),
	};

	let function_name = &input.sig.ident;
	let callback_name = LitStr::new(&input.sig.ident.to_string(), Span::call_site());

	// name of the linkme collector thats expected to be found at the root of the crate
	let collector_name = collector_name(&id);

	// also we need to get the type of the first argument, which is the the context
	// we need it to write out the type of the reactor
	let context_path = get_context_type(&input);

	quote! {
		const _: () = {
			#[#closureslop_path::__private_macroslop::linkme::distributed_slice(crate::#collector_name)]
			#[linkme(crate = #closureslop_path::__private_macroslop::linkme)]
			fn _add_callback(reactor: &mut #closureslop_path::Reactor<#context_path>) {
				#closureslop_path::add_callback!(reactor, #event => #callback_name => |ctx, args| {
					#closureslop_path::__private_macroslop::smallbox::SmallBox::new(async move {
						#function_name(ctx, args).await
					})
				}, #(before: #before,)* #(after: #after,)*);
			}
		};

		#input
	}
	.into()
}

/// Extracts the type of the first argument of the function
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

	match &*arg.ty {
		syn::Type::Reference(type_reference) => (*type_reference.elem).clone(),
		_ => panic!("context type must be a reference"),
	}
}
