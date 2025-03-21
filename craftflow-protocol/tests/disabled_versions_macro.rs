use craftflow_protocol::{
	c2s::handshaking::{SetProtocolBuilder, set_protocol::v5::SetProtocolV5},
	disabled_versions,
};

#[test]
fn disabled_features_macro() {
	let builder = SetProtocolBuilder::new(769);

	let built = match builder {
		SetProtocolBuilder::V5(p) => p.feed(SetProtocolV5 {
			protocol_version: 769,
			server_host: format!("127.0.0.1"),
			server_port: 25565,
			next_state: 5,
		}),
		disabled_versions!(c2s::handshaking::SetProtocolBuilder) => unreachable!(),
	};

	match built {
		craftflow_protocol::c2s::handshaking::SetProtocol::V5(_set_protocol_v5) => {
			//yay
		}
		disabled_versions!(c2s::handshaking::SetProtocol) => unreachable!(),
	}
}
