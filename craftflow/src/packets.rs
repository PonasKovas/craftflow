use craftflow_protocol_abstract::{c2s::AbC2S, s2c::AbS2C};
use craftflow_protocol_versions::{C2S, S2C};

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum C2SPacket {
	Abstract(AbC2S),
	Concrete(C2S),
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum S2CPacket {
	Abstract(AbS2C),
	Concrete(S2C),
}

impl C2SPacket {
	pub fn assume_concrete(self) -> C2S {
		match self {
			C2SPacket::Concrete(packet) => packet,
			_ => panic!("Packet is not concrete"),
		}
	}
	pub fn assume_abstract(self) -> AbC2S {
		match self {
			C2SPacket::Abstract(packet) => packet,
			_ => panic!("Packet is not abstract"),
		}
	}
}

impl S2CPacket {
	pub fn assume_concrete(self) -> S2C {
		match self {
			S2CPacket::Concrete(packet) => packet,
			_ => panic!("Packet is not concrete"),
		}
	}
	pub fn assume_abstract(self) -> AbS2C {
		match self {
			S2CPacket::Abstract(packet) => packet,
			_ => panic!("Packet is not abstract"),
		}
	}
}
