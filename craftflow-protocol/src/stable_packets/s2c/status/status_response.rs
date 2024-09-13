use crate::datatypes::Text;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StatusResponseInner<'a> {
	pub version: Version<'a>,
	#[serde(default)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub players: Option<Players<'a>>,
	#[serde(default)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<Text<'a>>,
	#[serde(default)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub favicon: Option<String>,
	#[serde(default)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub enforces_secure_chat: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Version<'a> {
	pub name: Cow<'a, str>,
	pub protocol: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Players<'a> {
	pub max: i32,
	pub online: i32,
	#[serde(default)]
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub sample: Vec<PlayerSample<'a>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PlayerSample<'a> {
	pub name: Cow<'a, str>,
	#[serde(serialize_with = "ser_uuid")]
	#[serde(deserialize_with = "de_uuid")]
	pub id: u128,
}

fn ser_uuid<S>(id: &u128, serializer: S) -> Result<S::Ok, S::Error>
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

fn de_uuid<'de, D>(deserializer: D) -> Result<u128, D::Error>
where
	D: serde::Deserializer<'de>,
{
	let s = String::deserialize(deserializer)?;
	let s = s.replace("-", "");
	u128::from_str_radix(&s, 16).map_err(serde::de::Error::custom)
}
