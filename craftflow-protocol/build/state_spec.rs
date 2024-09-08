use super::{util::AsTokenStream, version_bounds::Bounds};
use indexmap::IndexMap;
use proc_macro2::TokenStream;
use ron::extensions::Extensions;
use serde::Deserialize;
use std::{collections::BTreeMap, error::Error, fs, path::PathBuf};

// parses the state specification
pub fn parse_state_spec(path: &PathBuf) -> Result<StateSpec, Box<dyn Error>> {
	let file_contents = fs::read_to_string(path)?;
	let state_spec: StateSpec = ron::Options::default()
		.with_default_extension(Extensions::IMPLICIT_SOME | Extensions::UNWRAP_VARIANT_NEWTYPES)
		.from_str(&file_contents)?;

	Ok(state_spec)
}

#[derive(Deserialize, Debug, Clone)]
pub struct StateSpec {
	/// If needs a feature to be enabled
	pub feature: Option<String>,
	pub items: BTreeMap<String, SpecItem>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SpecItem {
	Packet(PacketSpec),
	Struct(StructSpec),
	Enum(EnumSpec),
}

#[derive(Deserialize, Debug, Clone)]
pub struct PacketSpec {
	/// If needs a feature to be enabled
	pub feature: Option<String>,
	/// The ID of the packet
	pub id: VersionDependent<i32>,
	pub data: IndexMap<String, Data>,
	pub format: Option<VersionDependent<Vec<FieldFormat>>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StructSpec {
	/// If needs a feature to be enabled
	pub feature: Option<String>,
	pub data: IndexMap<String, Data>,
	pub format: Option<VersionDependent<Vec<FieldFormat>>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct EnumSpec {
	/// If needs a feature to be enabled
	pub feature: Option<String>,
	pub variants: BTreeMap<String, EnumVariant>,
	/// Default tag_format is VarInt
	pub tag_format: Option<VersionDependent<TagFormat>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct EnumVariant {
	/// If needs a feature to be enabled
	pub feature: Option<String>,
	pub tag: VersionDependent<String>,
	pub data: Option<IndexMap<String, Data>>,
	pub format: Option<VersionDependent<Vec<FieldFormat>>>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum VersionDependent<T> {
	/// Shorthand for { "*": T }
	Always(T),
	/// Protocol version bounds -> T
	Map(IndexMap<Vec<Bounds>, T>),
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Data {
	/// A field that is always present
	Normal(String),
	/// A field that requires a feature
	RequiresFeature {
		feature: String,
		#[serde(rename = "type")]
		data_type: String,
		default: String,
	},
}

#[derive(Deserialize, Debug, Clone)]
pub struct FieldFormat {
	pub field: Option<String>,
	pub read_as: Option<String>,
	pub read: Option<String>,
	pub write: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TagFormat {
	pub read_as: Option<String>,
	pub read: Option<String>,
	pub write: Option<String>,
}

impl<T: Clone> VersionDependent<T> {
	pub fn expand_shortcut(&self) -> IndexMap<Vec<Bounds>, T> {
		match self {
			VersionDependent::Always(inner) => {
				let mut map = IndexMap::new();
				map.insert(vec![Bounds::All], inner.clone());

				map
			}
			VersionDependent::Map(map) => map.clone(),
		}
	}
}

// impl SpecItem {
// 	pub fn feature(&self) -> &Option<String> {
// 		match self {
// 			SpecItem::Packet(item) => &item.feature,
// 			SpecItem::Struct(item) => &item.feature,
// 			SpecItem::Enum(item) => &item.feature,
// 		}
// 	}
// }

// impl PacketSpec {
// 	pub fn fields(&self) -> Fields {
// 		Fields {
// 			data: &self.data,
// 			format: &self.format,
// 		}
// 	}
// }
// impl StructSpec {
// 	pub fn fields(&self) -> Fields {
// 		Fields {
// 			data: &self.data,
// 			format: &self.format,
// 		}
// 	}
// }
// impl EnumVariant {
// 	pub fn fields(&self) -> Option<Fields> {
// 		match &self.data {
// 			Some(data) => Some(Fields {
// 				data,
// 				format: &self.format,
// 			}),
// 			None => {
// 				if self.format.is_some() {
// 					panic!("Enum variant must not have `format` if doesn't have `data`");
// 				} else {
// 					None
// 				}
// 			}
// 		}
// 	}
// }

// impl Data {
// 	pub fn datatype(&self) -> TokenStream {
// 		match self {
// 			Data::Normal(t) => t.as_tokenstream(),
// 			Data::RequiresFeature {
// 				feature: _,
// 				data_type: t,
// 				default: _,
// 			} => t.as_tokenstream(),
// 		}
// 	}
// }
