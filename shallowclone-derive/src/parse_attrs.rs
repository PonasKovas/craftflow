use syn::{Attribute, Ident};

#[derive(Clone, Debug)]
pub struct Attributes {
	pub skip: bool,
	pub cow: bool,
	pub owned: bool,
	pub borrowed: bool,
}

impl Attributes {
	pub fn parse_item(attrs: &[Attribute]) -> Self {
		let attrs = Self::parse_all(attrs);
		assert_eq!(
			attrs.borrowed || attrs.owned,
			false,
			"owned/borrowed can only be used on variants of a cow enum"
		);
		assert_eq!(attrs.skip, false, "skip can only be used on generics");
		attrs
	}
	pub fn parse_variant(attrs: &[Attribute]) -> Self {
		let attrs = Self::parse_all(attrs);
		assert_eq!(attrs.skip, false, "skip can only be used on generics");
		assert_eq!(attrs.cow, false, "cow can only be used on enum item");
		attrs
	}
	pub fn parse_generic(attrs: &[Attribute]) -> Self {
		let attrs = Self::parse_all(attrs);
		assert_eq!(
			attrs.borrowed || attrs.owned,
			false,
			"owned/borrowed can only be used on variants of a cow enum"
		);
		assert_eq!(attrs.cow, false, "cow can only be used on enum item");
		attrs
	}
	fn parse_all(attrs: &[Attribute]) -> Self {
		let mut result = Self {
			skip: false,
			cow: false,
			owned: false,
			borrowed: false,
		};

		for attr in attrs {
			if let syn::Meta::List(list) = &attr.meta {
				if !list.path.is_ident("shallowclone") {
					continue;
				}

				if let Ok(parsed) = list.parse_args::<Ident>() {
					match parsed.to_string().as_str() {
						"skip" => result.skip = true,
						"cow" => result.cow = true,
						"owned" => result.owned = true,
						"borrowed" => result.borrowed = true,
						other => panic!("invalid attribute {other:?}"),
					}
					continue;
				}
			}
		}

		result
	}
}
