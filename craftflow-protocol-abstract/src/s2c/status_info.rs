use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor};
use anyhow::Result;
use craftflow_protocol_core::common_structures::Text;
use craftflow_protocol_versions::{
	s2c::{
		status::{server_info::v00765::ServerInfoV00005, ServerInfo},
		Status,
	},
	IntoStateEnum, S2C,
};
use serde::{Deserialize, Serialize};
use std::{
	borrow::Cow,
	iter::{once, Once},
};

/// Server status (MOTD, player count, favicon, etc.) sent in response to a [`AbStatusRequestInfo`] packet
#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbStatusInfo {
	/// The version info about the server (string name, and protocol version)
	pub version: Version,
	/// Player information, such as online/max and sample
	pub players: Option<Players>,
	/// The MOTD of the server
	pub description: Text,
	/// The favicon of the server, if any. This should be the raw PNG data.
	/// It must be exactly 64x64 pixels.
	#[serde(with = "favicon")]
	pub favicon: Option<Cow<'static, [u8]>>,
	#[serde(default)]
	pub enforces_secure_chat: bool,
}

/// Information about the version of the server.
#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Version {
	/// The text name of the version of the server.
	/// This has no logical significance, only for display.
	pub name: Text,
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

impl AbPacketWrite for AbStatusInfo {
	type Direction = S2C;
	type Iter = Once<Self::Direction>;

	fn convert(self, _protocol_version: u32) -> Result<Self::Iter> {
		// This packet is identical in all protocol versions

		Ok(once(
			ServerInfoV00005 {
				response: serde_json::to_string(&self)?,
			}
			.into_state_enum(),
		))
	}
}

impl AbPacketNew for AbStatusInfo {
	type Direction = S2C;
	type Constructor = NoConstructor<Self, S2C>;

	fn construct(
		packet: Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor, Self::Direction>> {
		match packet {
			S2C::Status(Status::ServerInfo(ServerInfo::V00005(packet))) => Ok(
				ConstructorResult::Done(serde_json::from_str(&packet.response)?),
			),
			_ => Ok(ConstructorResult::Ignore(packet)),
		}
	}
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
	use std::borrow::Cow;

	pub fn serialize<S>(id: &Option<Cow<'static, [u8]>>, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		match id {
			Some(id) => {
				let s = format!(
					"data:image/png;base64,{}",
					base64::prelude::BASE64_STANDARD.encode(id.as_ref())
				);

				serializer.serialize_str(&s)
			}
			None => serializer.serialize_none(),
		}
	}
	pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Cow<'static, [u8]>>, D::Error>
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

		Ok(Some(Cow::Owned(data)))
	}
}
