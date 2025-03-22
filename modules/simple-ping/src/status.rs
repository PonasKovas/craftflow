use crate::SimplePing;
use craftflow::CraftFlow;
use craftflow_protocol::{
	SUPPORTED_VERSIONS,
	c2s::status::PingStart,
	disabled_versions,
	s2c::status::{ServerInfoBuilder, server_info::v5::ServerInfoV5},
};
use serde::{Deserialize, Serialize};
use std::ops::ControlFlow;
use text::Text;

/// Server status (MOTD, player count, favicon, etc.) sent in response to a [`AbStatusRequestInfo`][crate::c2s::AbStatusRequestInfo] packet
#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusInfo {
	/// The version info about the server (string name, and protocol version)
	pub version: Version,
	/// Player information, such as online/max and sample
	#[serde(default)]
	pub players: Option<Players>,
	/// The MOTD of the server
	#[serde(default)]
	pub description: Option<Text<'static>>,
	/// The favicon of the server, if any. This should be the raw PNG data.
	/// It must be exactly 64x64 pixels.
	#[serde(with = "favicon")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub favicon: Option<Vec<u8>>,
	#[serde(default)]
	#[serde(skip_serializing_if = "std::ops::Not::not")]
	pub enforces_secure_chat: bool,
}

/// Information about the version of the server.
#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Version {
	/// The text name of the version of the server.
	/// This has no logical significance, only for display.
	/// You can use legacy formatting here (§c, §l, etc.)
	pub name: String,
	/// The protocol version of the server. If this doesn't match the client,
	/// the client will show "outdated server" or "outdated client" in the server list.
	pub protocol: u32,
}

/// Information about the players on the server.
#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Players {
	/// The maximum number of players that can connect to the server. Only for display.
	pub max: i32,
	/// The number of players currently connected to the server.
	pub online: i32,
	/// A sample of currently connected players. Shown, when the cursor is over the
	/// player count in the server list.
	#[serde(default)]
	pub sample: Vec<PlayerSample>,
}

/// An entry in the player sample list in [`Players`]
#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PlayerSample {
	/// The username of the player
	pub name: String,
	/// UUID of the player. Unused by the vanilla client.
	#[serde(with = "uuid")]
	pub id: u128,
}

#[craftflow::callback(event: PingStart)]
pub async fn status(
	cf: &CraftFlow,
	&mut (conn_id, ref mut _request): &mut (u64, PingStart),
) -> ControlFlow<()> {
	let version = cf.get(conn_id).protocol_version();

	// mirror client protocol version, or if not supported give one that is supported
	let version = if SUPPORTED_VERSIONS.contains(&version) {
		version
	} else {
		SUPPORTED_VERSIONS[0]
	};

	let online_players = cf.connections().len() as i32; // more or less (more)
	let max_players = 2_000_000_000; // todo after implementing max connections
	let description = cf.modules.get::<SimplePing>().server_description.clone();
	let favicon = cf.modules.get::<SimplePing>().favicon.clone();

	let status_info = StatusInfo {
		version: Version {
			name: format!("§f§lCraftFlow").into(),
			protocol: version,
		},
		players: Some(Players {
			max: max_players,
			online: online_players,
			sample: vec![], // todo real player sample
		}),
		description: Some(description),
		favicon,
		enforces_secure_chat: true,
	};

	let response = match ServerInfoBuilder::new(version) {
		disabled_versions!(s2c::status::ServerInfoBuilder) => unreachable!(),
		ServerInfoBuilder::V5(p) => p(ServerInfoV5 {
			response: serde_json::to_string(&status_info).expect("this cant fail bro"),
		}),
	};
	cf.get(conn_id).send(response).await;

	ControlFlow::Continue(())
}

mod uuid {
	use serde::Deserialize;

	pub fn serialize<S>(id: &u128, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		let s = format!(
			"{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
			(id >> (4 * 24)) & 0xffff_ffff,
			(id >> (4 * 20)) & 0xffff,
			(id >> (4 * 16)) & 0xffff,
			(id >> (4 * 12)) & 0xffff,
			id & 0xffff_ffff_ffff
		);
		serializer.serialize_str(&s)
	}
	pub fn deserialize<'de, D>(deserializer: D) -> Result<u128, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let s = String::deserialize(deserializer)?;
		let s = s.replace("-", "");
		u128::from_str_radix(&s, 16).map_err(serde::de::Error::custom)
	}
}

mod favicon {
	use base64::Engine;
	use serde::Deserialize;

	pub fn serialize<S>(favicon: &Option<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		match favicon {
			Some(favicon) => {
				let s = format!(
					"data:image/png;base64,{}",
					base64::prelude::BASE64_STANDARD.encode(&favicon)
				);

				serializer.serialize_str(&s)
			}
			None => serializer.serialize_none(),
		}
	}
	pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let s = String::deserialize(deserializer)?;
		let s = s.strip_prefix("data:image/png;base64,").ok_or_else(|| {
			serde::de::Error::custom("favicon must be a data URL with the MIME type image/png")
		})?;

		let data = base64::prelude::BASE64_STANDARD
			.decode(s.as_bytes())
			.map_err(|_| {
				serde::de::Error::custom("favicon must be a valid base64-encoded PNG image")
			})?;

		Ok(Some(data))
	}
}
