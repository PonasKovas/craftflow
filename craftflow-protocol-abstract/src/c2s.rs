/// All packets that can be sent from the client to the server
pub enum C2S {
	// There is only one packet in the Handshaking state and its identical in all protocol versions
	Handshake(craftflow_protocol_versions::v00005::c2s::handshaking::packet_set_protocol::PacketSetProtocol),
}
