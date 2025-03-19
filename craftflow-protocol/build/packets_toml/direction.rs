use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
	C2S,
	S2C,
}

impl Direction {
	pub fn new(s: &str) -> Self {
		match s {
			"s2c" => Self::S2C,
			"c2s" => Self::C2S,
			_ => panic!("bad direction {s:?}"),
		}
	}
	pub fn mod_name(self) -> &'static str {
		match self {
			Direction::C2S => "c2s",
			Direction::S2C => "s2c",
		}
	}
	pub fn enum_name(self) -> &'static str {
		match self {
			Direction::C2S => "C2S",
			Direction::S2C => "S2C",
		}
	}
}

impl Display for Direction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.mod_name())
	}
}
