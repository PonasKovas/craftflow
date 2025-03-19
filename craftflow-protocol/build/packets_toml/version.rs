use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Version(pub u32);

impl Version {
	pub fn mod_name(&self) -> String {
		format!("v{}", self.0)
	}
}

impl Display for Version {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.mod_name())
	}
}
