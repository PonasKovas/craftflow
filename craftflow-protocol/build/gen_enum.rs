use crate::DEFAULT_ENUM_DERIVES;

pub struct Variant {
	pub name: String,
	pub value: String,
}

pub fn gen_enum(name: &str, variants: &[Variant]) -> String {
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
	{DEFAULT_ENUM_DERIVES}
	pub enum {name} {{
        {variants_code}
    }}
	"#
	)
}
