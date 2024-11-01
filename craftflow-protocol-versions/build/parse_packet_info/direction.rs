#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
	C2S,
	S2C,
}

impl Direction {
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
