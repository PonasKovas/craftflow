use proc_macro2::TokenStream;

use crate::build::{
	state_spec::{FieldFormat, TagFormat},
	util::AsTokenStream,
};

#[derive(Debug, Default)]
pub struct CustomFormat {
	pub custom_read: Option<CustomRead>,
	pub custom_write: Option<TokenStream>,
}

#[derive(Debug)]
pub struct CustomRead {
	pub read_as: TokenStream,
	pub read: TokenStream,
}

impl CustomFormat {
	pub fn from_field_format(spec: &FieldFormat) -> Self {
		let custom_read = match (&spec.read_as, &spec.read) {
			(None, None) => None,
			(None, Some(_)) | (Some(_), None) => panic!(
				"A field cannot have `read` or `read_as` by itself, they must come together!"
			),
			(Some(read_as), Some(read)) => Some(CustomRead {
				read_as: read_as.as_tokenstream(),
				read: read.as_tokenstream(),
			}),
		};

		CustomFormat {
			custom_read,
			custom_write: spec.write.as_ref().map(|w| w.as_tokenstream()),
		}
	}
	pub fn from_tag_format(spec: &TagFormat) -> Self {
		let custom_read = match (&spec.read_as, &spec.read) {
			(None, None) => None,
			(None, Some(_)) | (Some(_), None) => panic!(
				"A field cannot have `read` or `read_as` by itself, they must come together!"
			),
			(Some(read_as), Some(read)) => Some(CustomRead {
				read_as: read_as.as_tokenstream(),
				read: read.as_tokenstream(),
			}),
		};

		CustomFormat {
			custom_read,
			custom_write: spec.write.as_ref().map(|w| w.as_tokenstream()),
		}
	}
}
