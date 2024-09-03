use crate::datatypes::{TextJSON, VarInt};

impl_mcp_traits! {
	C2S: LoginC2S;
	[0] LoginStart {
		name: String,
		uuid: u128,
	}
	[1] EncryptionResponse {
		shared_secret: Vec<u8>,
		verify_token: Vec<u8>,
	}
	[2] PluginResponse {
		message_id: VarInt,
		success: bool,
		data: Box<[u8]>,
	}
	[3] LoginAcknowledged {}
	[4] CookieResponse {
		key: String,
		payload: Option<Vec<u8>>,
	}
}

impl_mcp_traits! {
	S2C: LoginS2C;
	[0] Disconnect {
		reason: TextJSON,
	}
	[1] EncryptionRequest {
		server_id: String,
		public_key: Vec<u8>,
		verify_token: Vec<u8>,
		should_authenticate: bool,
	}
	[2] LoginSuccess {
		uuid: u128,
		username: String,
		properties: Vec<Property>,
		strict_error_handling: bool,
	}
	[3] SetCompression {
		threshold: VarInt,
	}
	[4] PluginRequest {
		message_id: VarInt,
		channel: String,
		data: Box<[u8]>,
	}
	[5] CookieRequest {
		key: String,
	}
}

mcp_struct! {
	Property {
		name: String,
		value: String,
		signature: Option<String>,
	}
}
