use craftflow_protocol::{
	c2s::handshaking::{SetProtocolBuilder, set_protocol::v5::SetProtocolV5},
	disabled_versions,
};

#[test]
fn disabled_features_macro() {
	let builder = SetProtocolBuilder::new(769);

	let built = match builder {
		disabled_versions!(c2s::handshaking::SetProtocolBuilder) => unreachable!(),
		SetProtocolBuilder::V5(p) => p(SetProtocolV5 {
			protocol_version: 769,
			server_host: format!("127.0.0.1"),
			server_port: 25565,
			next_state: 5,
		}),
	};

	match built {
		disabled_versions!(c2s::handshaking::SetProtocol) => unreachable!(),
		craftflow_protocol::c2s::handshaking::SetProtocol::V5(_set_protocol_v5) => {
			//yay
		}
	}
}
