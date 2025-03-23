use super::Version;
use crate::shared::snake_to_pascal_case;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Type(pub String);

impl Type {
	pub fn mod_name(&self) -> &str {
		&self.0
	}
	pub fn enum_name(&self) -> String {
		snake_to_pascal_case(&self.0)
	}
	pub fn struct_name(&self, version: Version) -> String {
		format!("{}V{}", snake_to_pascal_case(&self.0), version.0)
	}
}

impl Display for Type {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.mod_name())
	}
}
