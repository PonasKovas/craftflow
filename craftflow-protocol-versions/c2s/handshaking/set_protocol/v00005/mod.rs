define_type! {
	#[derive(ShallowClone, Debug, PartialEq, Clone, Hash)]
	pub struct SetProtocolV00005<'a> {
		pub protocol_version: VarInt,
		pub server_host: Cow<'a, str>,
		pub server_port: u16,
		pub next_state: VarInt,
	}
}
