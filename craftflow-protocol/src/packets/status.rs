use crate::{datatypes::Text, MCPWritable};
use serde::{Deserialize, Serialize};

impl_mcp_traits! {
	C2S: StatusC2S;
	[0] StatusRequest {}
	[1] Ping {
		payload: i64,
	}
}

impl_mcp_traits! {
	S2C: StatusS2C;
	[0] StatusResponse {
		response: StatusResponseJSON,
	}
	[1] Pong {
		payload: i64,
	}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StatusResponseJSON {
	pub version: Version,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub players: Option<Players>,
	pub description: Text,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub favicon: Option<String>,
	#[serde(rename = "enforcesSecureChat")]
	#[serde(default)]
	#[serde(skip_serializing_if = "std::ops::Not::not")]
	pub enforces_secure_chat: bool,
}

impl MCPWritable for StatusResponseJSON {
	fn write(&self, to: &mut impl std::io::Write) -> anyhow::Result<usize> {
		let s = serde_json::to_string(self)?;

		s.write(to)
	}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Version {
	pub name: String,
	pub protocol: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Players {
	pub max: i32,
	pub online: i32,
	pub sample: Vec<PlayerSample>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerSample {
	pub name: String,
	pub id: String,
}
