use crate::parse_packet_info::Generics;

pub struct Variant {
	pub name: String,
	pub value_path: String,
	pub value_generics: Generics,
}

pub fn gen_enum(name: &str, variants: &[Variant]) -> String {
	let enum_generics = variants
		.iter()
		.fold(Generics::new(), |acc, g| acc.union(&g.value_generics));

	let enum_generics = enum_generics.as_str();

	let mut variants_code = String::new();

	for variant in variants {
		variants_code.push_str(&format!(
			r#"{name}({value_path} {variant_generics}),
			"#,
			name = variant.name,
			value_path = variant.value_path,
			variant_generics = variant.value_generics.as_str(),
		));
	}

	format!(
		r#"
	#[derive(ShallowClone, Debug, PartialEq, Clone)]
	pub enum {name} {enum_generics} {{
        {variants_code}
    }}
	"#
	)
}
