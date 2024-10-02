use craftflow_protocol_abstract::{c2s::AbC2S, s2c::AbS2C};
use craftflow_protocol_versions::{C2S, S2C};

pub enum C2SPacket {
	Abstract(AbC2S),
	Concrete(C2S),
}

pub enum S2CPacket {
	Abstract(AbS2C),
	Concrete(S2C),
}
