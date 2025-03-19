use crate::shared::snake_to_pascal_case;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct State(pub String);

impl State {
	pub fn mod_name(&self) -> &str {
		&self.0
	}
	pub fn enum_name(&self) -> String {
		snake_to_pascal_case(&self.0)
	}
}

impl Display for State {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.mod_name())
	}
}
