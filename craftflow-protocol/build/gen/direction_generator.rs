use super::state_generator::StateGenerator;
use crate::build::{gen::feature::FeatureCfgOptions, util::Direction};
use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub struct DirectionGenerator {
	pub direction: Direction,
	pub states: Vec<StateGenerator>,
}

impl DirectionGenerator {
	/// Generates the direction module
	/// This includes a module and an enum for each state
	pub fn gen(&self) -> TokenStream {
		let mut result = TokenStream::new();

		for state in &self.states {
			result.extend(state.gen());
		}

		result
	}
}
