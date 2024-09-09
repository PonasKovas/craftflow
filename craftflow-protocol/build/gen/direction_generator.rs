use super::state_generator::StateGenerator;
use crate::build::{info_file::Info, util::Direction};
use proc_macro2::TokenStream;
use quote::quote;

pub struct DirectionGenerator {
	pub direction: Direction,
	pub states: Vec<StateGenerator>,
}

impl DirectionGenerator {
	/// Generates the direction module
	/// This includes a module and an enum for each state
	pub fn gen(&self, info: &Info) -> TokenStream {
		let mut result = TokenStream::new();

		let direction_module = self.direction.module();
		result.extend(quote! {
			pub use crate::stable_packets::#direction_module::*;
		});

		for state in &self.states {
			result.extend(state.gen(info));
		}

		result
	}
}
