mod macros;

use crate::MinecraftProtocol;
use serde::{Deserialize, Serialize};
use std::{
	io::Read,
	ops::{Add, AddAssign},
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Text {
	String(String),
	Array(Vec<Text>),
	Object(Box<TextObject>),
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TextObject {
	#[serde(flatten)]
	pub content: TextContent,
	#[serde(default)]
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub extra: Vec<Text>,
	/// The text color, which may be a color name or a #-prefixed hexadecimal RGB specification
	#[serde(default)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub color: Option<String>,
	#[serde(default)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bold: Option<bool>,
	#[serde(default)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub italic: Option<bool>,
	#[serde(default)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub underlined: Option<bool>,
	#[serde(default)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub strikethrough: Option<bool>,
	#[serde(default)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub obfuscated: Option<bool>,
	#[serde(default)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub font: Option<String>,
	#[serde(default)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub insertion: Option<String>,
	#[serde(default)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub click_event: Option<ClickEvent>,
	#[serde(default)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub hover_event: Option<HoverEvent>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum TextContent {
	Text {
		/// Set as the content directly, with no additional processing.
		text: String,
	},
	Translate {
		/// A translation key, looked up in the current language file to obtain a translation text, which
		/// becomes the component's content after processing.
		translate: String,
		/// Replacements for placeholders in the translation text.
		#[serde(default)]
		#[serde(skip_serializing_if = "Option::is_none")]
		with: Option<Vec<Text>>,
	},
	Keybind {
		/// The name of a keybinding. The client's current setting for the specified keybinding becomes the component's content.
		/// The value is named after the keys in options.txt (for instance, for key_key.forward in options.txt, key.forward would
		/// be used in the component and W would be displayed).
		keybind: String,
	},
	Score {
		score: Score,
	},
	Selector {
		/// An entity selector.
		selector: String,
		/// Separator to place between results. If omitted, defaults to {"color":"gray","text":", "}
		#[serde(default)]
		#[serde(skip_serializing_if = "Option::is_none")]
		separator: Option<Text>,
	},
	Nbt {
		/// NBT path to be queried.
		nbt: String,
		#[serde(default)]
		#[serde(skip_serializing_if = "std::ops::Not::not")]
		/// If true, the server will attempt to parse and resolve each result as (an NBT string containing) a text component for display
		interpret: bool,
		/// Separator to place between results. If omitted, defaults to {"text":", "}.
		#[serde(default)]
		#[serde(skip_serializing_if = "Option::is_none")]
		separator: Option<Text>,
		data_source: TextNbtDataSource,
	},
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum TextNbtDataSource {
	Block {
		/// Location of a block entity to be queried, in the usual space-separated coordinate syntax with support for ~ and ^.
		block: String,
	},
	Entity {
		/// Selector specifying the set of entities to be queried.
		entity: String,
	},
	Storage {
		/// Identifier specifying the storage to be queried.
		storage: String,
	},
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Score {
	/// A player username, player or entity UUID, entity selector (that selects one entity), or * to match the sending player.
	pub name: String,
	/// The name of the objective.
	pub objective: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ClickEvent {
	pub action: ClickEventAction,
	pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ClickEventAction {
	OpenUrl,
	RunCommand,
	SuggestCommand,
	ChangePage,
	CopyToClipboard,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct HoverEvent {
	pub action: HoverEventAction,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum HoverEventAction {
	ShowText {
		#[serde(flatten)]
		contents: Text,
	},
	ShowItem {
		#[serde(flatten)]
		contents: HoverActionShowItem,
	},
	ShowEntity {
		#[serde(flatten)]
		contents: HoverActionShowEntity,
	},
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct HoverActionShowItem {
	/// The textual identifier of the item's type. If unrecognized, defaults to minecraft:air.
	pub id: String,
	/// The number of items in the item stack.
	#[serde(default)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub count: Option<i32>,
	/// The item's NBT information as sNBT (as would be used in /give)
	#[serde(default)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tag: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct HoverActionShowEntity {
	/// The textual identifier of the entity's type. If unrecognized, defaults to minecraft:pig.
	#[serde(rename = "type")]
	pub entity_type: String,
	/// The entity's UUID (with dashes). Does not need to correspond to an existing entity; only for display.
	pub id: String,
	/// The entity's custom name.
	#[serde(default)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
}

impl MinecraftProtocol for Text {
	fn read(protocol_version: u32, source: &mut impl Read) -> anyhow::Result<Self> {
		Ok(serde_json::from_str(&String::read(
			protocol_version,
			source,
		)?)?)
	}
	fn write(&self, protocol_version: u32, to: &mut impl std::io::Write) -> anyhow::Result<usize> {
		let s = serde_json::to_string(self)?;

		s.write(protocol_version, to)
	}
}

impl Add for Text {
	type Output = Text;

	fn add(self, rhs: Self) -> Self::Output {
		Text::Array(vec![
			// the first one counts as the parent of the following ones, so we add an empty one
			// to not change any styles
			Text::String("".to_string()),
			self,
			rhs,
		])
	}
}

impl Add<&Text> for Text {
	type Output = Text;

	fn add(self, rhs: &Self) -> Self::Output {
		Text::Array(vec![
			// the first one counts as the parent of the following ones, so we add an empty one
			// to not change any styles
			Text::String("".to_string()),
			self,
			rhs.clone(),
		])
	}
}

impl AddAssign for Text {
	fn add_assign(&mut self, rhs: Self) {
		// will the compiler optimize this clone?
		// 🤷
		*self = self.clone() + rhs;
	}
}

impl AddAssign<&Text> for Text {
	fn add_assign(&mut self, rhs: &Self) {
		*self = self.clone() + rhs.clone();
	}
}

impl Default for TextContent {
	fn default() -> Self {
		TextContent::Text {
			text: "".to_string(),
		}
	}
}
