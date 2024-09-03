use std::io::Read;

use crate::{MCPReadable, MCPWritable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum TextJSON {
	String(String),
	Array(Vec<TextJSON>),
	Object(Box<TextObjectJSON>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TextObjectJSON {
	#[serde(flatten)]
	content: TextContentJSON,
	/// The text color, which may be a color name or a #-prefixed hexadecimal RGB specification
	#[serde(skip_serializing_if = "Option::is_none")]
	color: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	bold: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	italic: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	underlined: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	strikethrough: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	obfuscated: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	font: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	insertion: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	click_event: Option<ClickEventJSON>,
	#[serde(skip_serializing_if = "Option::is_none")]
	hover_event: Option<HoverEventJSON>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum TextContentJSON {
	Text {
		/// Set as the content directly, with no additional processing.
		text: String,
	},
	Translate {
		/// A translation key, looked up in the current language file to obtain a translation text, which
		/// becomes the component's content after processing.
		translate: String,
		/// Replacements for placeholders in the translation text.
		#[serde(skip_serializing_if = "Option::is_none")]
		with: Option<Vec<TextJSON>>,
	},
	Keybind {
		/// The name of a keybinding. The client's current setting for the specified keybinding becomes the component's content.
		/// The value is named after the keys in options.txt (for instance, for key_key.forward in options.txt, key.forward would
		/// be used in the component and W would be displayed).
		keybind: String,
	},
	Score {
		score: ScoreJSON,
	},
	Selector {
		/// An entity selector.
		selector: String,
		/// Separator to place between results. If omitted, defaults to {"color":"gray","text":", "}
		#[serde(skip_serializing_if = "Option::is_none")]
		separator: Option<TextJSON>,
	},
	Nbt {
		/// NBT path to be queried.
		nbt: String,
		#[serde(default)]
		#[serde(skip_serializing_if = "std::ops::Not::not")]
		/// If true, the server will attempt to parse and resolve each result as (an NBT string containing) a text component for display
		interpret: bool,
		/// Separator to place between results. If omitted, defaults to {"text":", "}.
		#[serde(skip_serializing_if = "Option::is_none")]
		separator: Option<TextJSON>,
		data_source: TextNbtDataSourceJSON,
	},
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum TextNbtDataSourceJSON {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScoreJSON {
	/// A player username, player or entity UUID, entity selector (that selects one entity), or * to match the sending player.
	name: String,
	/// The name of the objective.
	objective: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClickEventJSON {
	action: ClickEventActionJSON,
	value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ClickEventActionJSON {
	OpenUrl,
	RunCommand,
	SuggestCommand,
	ChangePage,
	CopyToClipboard,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HoverEventJSON {
	action: HoverEventActionJSON,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum HoverEventActionJSON {
	ShowText {
		#[serde(flatten)]
		contents: TextJSON,
	},
	ShowItem {
		#[serde(flatten)]
		contents: HoverActionShowItemJSON,
	},
	ShowEntity {
		#[serde(flatten)]
		contents: HoverActionShowEntityJSON,
	},
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HoverActionShowItemJSON {
	/// The textual identifier of the item's type. If unrecognized, defaults to minecraft:air.
	id: String,
	/// The number of items in the item stack.
	#[serde(skip_serializing_if = "Option::is_none")]
	count: Option<i32>,
	/// The item's NBT information as sNBT (as would be used in /give)
	#[serde(skip_serializing_if = "Option::is_none")]
	tag: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HoverActionShowEntityJSON {
	/// The textual identifier of the entity's type. If unrecognized, defaults to minecraft:pig.
	#[serde(rename = "type")]
	entity_type: String,
	/// The entity's UUID (with dashes). Does not need to correspond to an existing entity; only for display.
	id: String,
	/// The entity's custom name.
	#[serde(skip_serializing_if = "Option::is_none")]
	name: Option<String>,
}

impl MCPWritable for TextJSON {
	fn write(&self, to: &mut impl std::io::Write) -> anyhow::Result<usize> {
		let s = serde_json::to_string(self)?;

		s.write(to)
	}
}
impl MCPReadable for TextJSON {
	fn read(source: &mut impl Read) -> anyhow::Result<Self> {
		Ok(serde_json::from_str(&String::read(source)?)?)
	}
}
