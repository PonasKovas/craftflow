use super::{Direction, State, Version};
use crate::shared::snake_to_pascal_case;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
	Common(String),
	Specific {
		direction: Direction,
		state: State,
		name: String,
	},
}

impl Type {
	fn last(&self) -> &str {
		match self {
			Type::Common(s) => s,
			Type::Specific {
				direction: _,
				state: _,
				name,
			} => name,
		}
	}
	pub fn parts(&self) -> Vec<String> {
		match self {
			Type::Common(name) => vec![name.to_owned()],
			Type::Specific {
				direction,
				state,
				name,
			} => vec![
				direction.mod_name().to_owned(),
				state.mod_name().to_owned(),
				name.clone(),
			],
		}
	}
	pub fn enum_name(&self) -> String {
		snake_to_pascal_case(self.last())
	}
	pub fn struct_name(&self, version: Version) -> String {
		format!("{}V{}", snake_to_pascal_case(self.last()), version.0)
	}
}

impl Display for Type {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.parts().join("::"))
	}
}
