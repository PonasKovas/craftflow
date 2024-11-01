pub struct Variant {
	pub name: String,
	pub value_path: String,
	pub has_lifetime: bool,
}

pub fn gen_enum(name: &str, variants: &[Variant]) -> String {
	let has_lifetime = variants.iter().any(|v| v.has_lifetime);
	let lifetime = if has_lifetime { "<'a>" } else { "" };

	let mut variants_code = String::new();

	for variant in variants {
		variants_code.push_str(&format!(
			r#"{name}({value_path} {lifetime}),
			"#,
			name = variant.name,
			value_path = variant.value_path,
			lifetime = if variant.has_lifetime { "<'a>" } else { "" }
		));
	}

	format!(
		r#"
	pub enum {name} {lifetime} {{
        {variants_code}
    }}
	"#
	)
}
