mod macros;

use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::ops::{Add, AddAssign};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Text<'a> {
	String(Cow<'a, str>),
	Array(Vec<Text<'a>>),
	Object(Box<TextObject<'a>>),
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TextObject<'a> {
	#[serde(flatten)]
	pub content: TextContent<'a>,
	#[serde(default)]
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub extra: Vec<Text<'a>>,
	/// The text color, which may be a color name or a #-prefixed hexadecimal RGB specification
	#[serde(default)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub color: Option<Cow<'a, str>>,
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
	pub font: Option<Cow<'a, str>>,
	#[serde(default)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub insertion: Option<Cow<'a, str>>,
	#[serde(default)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub click_event: Option<ClickEvent<'a>>,
	#[serde(default)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub hover_event: Option<HoverEvent<'a>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum TextContent<'a> {
	Text {
		/// Set as the content directly, with no additional processing.
		text: Cow<'a, str>,
	},
	Translate {
		/// A translation key, looked up in the current language file to obtain a translation text, which
		/// becomes the component's content after processing.
		translate: Cow<'a, str>,
		/// Replacements for placeholders in the translation text.
		#[serde(default)]
		#[serde(skip_serializing_if = "Option::is_none")]
		with: Option<Vec<Text<'a>>>,
	},
	Keybind {
		/// The name of a keybinding. The client's current setting for the specified keybinding becomes the component's content.
		/// The value is named after the keys in options.txt (for instance, for key_key.forward in options.txt, key.forward would
		/// be used in the component and W would be displayed).
		keybind: Cow<'a, str>,
	},
	Score {
		score: Score<'a>,
	},
	Selector {
		/// An entity selector.
		selector: Cow<'a, str>,
		/// Separator to place between results. If omitted, defaults to {"color":"gray","text":", "}
		#[serde(default)]
		#[serde(skip_serializing_if = "Option::is_none")]
		separator: Option<Text<'a>>,
	},
	Nbt {
		/// NBT path to be queried.
		nbt: Cow<'a, str>,
		#[serde(default)]
		#[serde(skip_serializing_if = "std::ops::Not::not")]
		/// If true, the server will attempt to parse and resolve each result as (an NBT string containing) a text component for display
		interpret: bool,
		/// Separator to place between results. If omitted, defaults to {"text":", "}.
		#[serde(default)]
		#[serde(skip_serializing_if = "Option::is_none")]
		separator: Option<Text<'a>>,
		data_source: TextNbtDataSource<'a>,
	},
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum TextNbtDataSource<'a> {
	Block {
		/// Location of a block entity to be queried, in the usual space-separated coordinate syntax with support for ~ and ^.
		block: Cow<'a, str>,
	},
	Entity {
		/// Selector specifying the set of entities to be queried.
		entity: Cow<'a, str>,
	},
	Storage {
		/// Identifier specifying the storage to be queried.
		storage: Cow<'a, str>,
	},
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Score<'a> {
	/// A player username, player or entity UUID, entity selector (that selects one entity), or * to match the sending player.
	pub name: Cow<'a, str>,
	/// The name of the objective.
	pub objective: Cow<'a, str>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ClickEvent<'a> {
	pub action: ClickEventAction,
	pub value: Cow<'a, str>,
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
pub struct HoverEvent<'a> {
	pub action: HoverEventAction<'a>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum HoverEventAction<'a> {
	ShowText {
		#[serde(flatten)]
		contents: Text<'a>,
	},
	ShowItem {
		#[serde(flatten)]
		contents: HoverActionShowItem<'a>,
	},
	ShowEntity {
		#[serde(flatten)]
		contents: HoverActionShowEntity<'a>,
	},
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct HoverActionShowItem<'a> {
	/// The textual identifier of the item's type. If unrecognized, defaults to minecraft:air.
	pub id: Cow<'a, str>,
	/// The number of items in the item stack.
	#[serde(default)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub count: Option<i32>,
	/// The item's NBT information as sNBT (as would be used in /give)
	#[serde(default)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tag: Option<Cow<'a, str>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct HoverActionShowEntity<'a> {
	/// The textual identifier of the entity's type. If unrecognized, defaults to minecraft:pig.
	#[serde(rename = "type")]
	pub entity_type: Cow<'a, str>,
	/// The entity's UUID (with dashes). Does not need to correspond to an existing entity; only for display.
	pub id: Cow<'a, str>,
	/// The entity's custom name.
	#[serde(default)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<Cow<'a, str>>,
}

impl<'a> Add for Text<'a> {
	type Output = Text<'a>;

	fn add(self, rhs: Self) -> Self::Output {
		Text::Array(vec![
			// the first one counts as the parent of the following ones, so we add an empty one
			// to not change any styles
			Text::String("".into()),
			self,
			rhs,
		])
	}
}

impl<'a> Add<&Text<'a>> for Text<'a> {
	type Output = Text<'a>;

	fn add(self, rhs: &Self) -> Self::Output {
		Text::Array(vec![
			// the first one counts as the parent of the following ones, so we add an empty one
			// to not change any styles
			Text::String("".into()),
			self,
			rhs.clone(),
		])
	}
}

impl<'a> AddAssign for Text<'a> {
	fn add_assign(&mut self, rhs: Self) {
		// will the compiler optimize this clone?
		// 🤷
		*self = self.clone() + rhs;
	}
}

impl<'a> AddAssign<&Text<'a>> for Text<'a> {
	fn add_assign(&mut self, rhs: &Self) {
		*self = self.clone() + rhs.clone();
	}
}

impl<'a> Default for TextContent<'a> {
	fn default() -> Self {
		TextContent::Text { text: "".into() }
	}
}
