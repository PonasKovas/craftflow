use super::state_generator::StateGenerator;
use crate::build::info_file::Info;
use proc_macro2::TokenStream;

pub struct DirectionGenerator {
	pub states: Vec<StateGenerator>,
}

impl DirectionGenerator {
	/// Generates the direction module
	/// This includes a module and an enum for each state
	pub fn gen(&self, info: &Info) -> TokenStream {
		let mut result = TokenStream::new();

		for state in &self.states {
			result.extend(state.gen(info));
		}

		result
	}
}
