use crate::datatypes::VarInt;

impl_mcp_traits! {
	C2S: HandshakeC2S;
	[0] Handshake {
		protocol_version: VarInt,
		server_address: String,
		server_port: u16,
		next_state: NextState,
	}
}

varint_enum! {
	NextState {
		Status = 1,
		Login = 2,
		Transfer = 3,
	}
}
