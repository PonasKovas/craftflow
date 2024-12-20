use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, State, WriteResult};
use anyhow::Result;
use craftflow_protocol_core::common_structures::Text;
use craftflow_protocol_versions::{
	s2c::{
		status::{server_info::v00005::ServerInfoV00005, ServerInfo},
		Status,
	},
	IntoStateEnum, S2C,
};
use serde::{Deserialize, Serialize};
use shallowclone::{MakeOwned, ShallowClone};
use std::{
	borrow::Cow,
	iter::{once, Once},
};

/// Server status (MOTD, player count, favicon, etc.) sent in response to a [`AbStatusRequestInfo`][crate::c2s::AbStatusRequestInfo] packet
#[derive(
	ShallowClone,
	MakeOwned,
	Debug,
	Clone,
	PartialEq,
	Hash,
	Eq,
	PartialOrd,
	Ord,
	Serialize,
	Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct AbStatusInfo<'a> {
	/// The version info about the server (string name, and protocol version)
	pub version: Version<'a>,
	/// Player information, such as online/max and sample
	#[serde(default)]
	pub players: Option<Players<'a>>,
	/// The MOTD of the server
	#[serde(default)]
	pub description: Option<Text<'a>>,
	/// The favicon of the server, if any. This should be the raw PNG data.
	/// It must be exactly 64x64 pixels.
	#[serde(with = "favicon")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub favicon: Option<Cow<'a, [u8]>>,
	#[serde(default)]
	#[serde(skip_serializing_if = "std::ops::Not::not")]
	pub enforces_secure_chat: bool,
}

/// Information about the version of the server.
#[derive(
	ShallowClone,
	MakeOwned,
	Debug,
	Clone,
	PartialEq,
	Hash,
	Eq,
	PartialOrd,
	Ord,
	Serialize,
	Deserialize,
)]
pub struct Version<'a> {
	/// The text name of the version of the server.
	/// This has no logical significance, only for display.
	/// You can use legacy formatting here (§c, §l, etc.)
	pub name: Cow<'a, str>,
	/// The protocol version of the server. If this doesn't match the client,
	/// the client will show "outdated server" or "outdated client" in the server list.
	pub protocol: u32,
}

/// Information about the players on the server.
#[derive(
	ShallowClone,
	MakeOwned,
	Debug,
	Clone,
	PartialEq,
	Hash,
	Eq,
	PartialOrd,
	Ord,
	Serialize,
	Deserialize,
)]
pub struct Players<'a> {
	/// The maximum number of players that can connect to the server. Only for display.
	pub max: i32,
	/// The number of players currently connected to the server.
	pub online: i32,
	/// A sample of currently connected players. Shown, when the cursor is over the
	/// player count in the server list.
	#[serde(default)]
	pub sample: Vec<PlayerSample<'a>>,
}

/// An entry in the player sample list in [`Players`]
#[derive(
	ShallowClone,
	MakeOwned,
	Debug,
	Clone,
	PartialEq,
	Hash,
	Eq,
	PartialOrd,
	Ord,
	Serialize,
	Deserialize,
)]
pub struct PlayerSample<'a> {
	/// The username of the player
	pub name: Cow<'a, str>,
	/// UUID of the player. Unused by the vanilla client.
	#[serde(with = "uuid")]
	pub id: u128,
}

impl<'a> AbPacketWrite<'a> for AbStatusInfo<'a> {
	type Direction = S2C<'a>;
	type Iter = Once<Self::Direction>;

	fn convert(&'a self, _protocol_version: u32, state: State) -> Result<WriteResult<Self::Iter>> {
		if state != State::Status {
			return Ok(WriteResult::Unsupported);
		}

		// This packet is identical in all protocol versions
		Ok(WriteResult::Success(once(
			ServerInfoV00005 {
				response: Cow::Owned(serde_json::to_string(&self)?),
			}
			.into_state_enum(),
		)))
	}
}

impl<'a> AbPacketNew<'a> for AbStatusInfo<'a> {
	type Direction = S2C<'a>;
	type Constructor = NoConstructor<AbStatusInfo<'static>>;

	fn construct(
		packet: &'a Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor>> {
		match packet {
			S2C::Status(Status::ServerInfo(ServerInfo::V00005(packet))) => Ok(
				ConstructorResult::Done(serde_json::from_str(&packet.response)?),
			),
			_ => Ok(ConstructorResult::Ignore),
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

	pub fn serialize<'a, S>(id: &Option<Cow<'a, [u8]>>, serializer: S) -> Result<S::Ok, S::Error>
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
