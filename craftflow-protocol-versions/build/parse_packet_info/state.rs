use crate::common::snake_to_pascal_case;

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
