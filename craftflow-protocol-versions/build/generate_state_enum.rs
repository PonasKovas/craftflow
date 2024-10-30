use crate::common::snake_to_pascal_case;

pub fn generate_state_enum(direction: &str, states: &Vec<String>) -> String {
	let enum_name = direction.to_uppercase();

	let mut enum_variants = String::new();
	for state in states {
		let variant_name = snake_to_pascal_case(state);

		enum_variants += &format!("{variant_name}({direction}::{variant_name}<'a>),\n");
	}

	format!(
		"pub use {direction}_enum::*;
		mod {direction}_enum {{
		use super::*;

		#[derive(Debug, PartialEq, Clone)]
		pub enum {enum_name}<'a> {{
            {enum_variants}
        }}

        impl<'a> crate::IntoStateEnum for {enum_name}<'a> {{
            type Direction = Self;

           	fn into_state_enum(self) -> Self::Direction {{
                self
            }}
        }}
        }}"
	)
}
