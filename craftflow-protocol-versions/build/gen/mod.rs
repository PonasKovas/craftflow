use super::json_defs::ProtocolFile;
use proc_macro2::TokenStream;
use quote::quote;
use std::{collections::HashMap, error::Error};

pub fn generate(defs: HashMap<u32, ProtocolFile>) -> Result<TokenStream, Box<dyn Error>> {
	Ok(quote! {})
}
