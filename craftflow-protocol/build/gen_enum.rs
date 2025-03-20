use crate::DEFAULT_ENUM_DERIVES;

pub struct Variant {
	pub name: String,
	pub value: String,
}

pub fn gen_enum(name: &str, variants: &[Variant], default_derives: bool) -> String {
	let mut variants_code = String::new();

	for variant in variants {
		variants_code.push_str(&format!(
			"{name}({value_path}),\n",
			name = variant.name,
			value_path = variant.value,
		));
	}

	format!(
		r#"
	{derives}
	pub enum {name} {{
        {variants_code}
    }}
	"#,
		derives = if default_derives {
			DEFAULT_ENUM_DERIVES
		} else {
			""
		}
	)
}
