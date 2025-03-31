use crate::{
	DEFAULT_IMPORTS_FOR_IMPLS, TYPES_DIR,
	packets_toml::{Direction, PacketsToml, Type},
	shared::package_dir,
};
use indexmap::IndexMap;

pub fn generate(pkts_toml: &PacketsToml) -> String {
	let mut code = String::new();

	// first load the actual implementations in a clean-for-the-user way
	for (ty, all_version_groups) in &pkts_toml.types {
		let mut type_code = String::new();
		for (&group_id, _versions) in all_version_groups {
			let mut impl_path = package_dir().join(TYPES_DIR);
			for part in ty.parts() {
				impl_path = impl_path.join(part);
			}
			impl_path = impl_path.join(format!("{}.rs", group_id.mod_name()));

			let impl_path = impl_path.to_str().expect("impl path not valid utf8");

			type_code += &format!(
				r#"
				pub mod {group_id} {{
					{DEFAULT_IMPORTS_FOR_IMPLS}
					include!{{ "{impl_path}" }}
				}}
				"#
			);
		}

		let mut inner_code = type_code;
		let mut parts = ty.parts();
		parts.reverse();
		for part in parts {
			inner_code = format!("pub mod {part} {{ {inner_code} }}");
		}
		code += &inner_code;
	}

	// then generate private version modules and reexports for internal usage
	// for easy ALL types import for a specific version
	// use crate::types::vXXX::*;
	for version in &pkts_toml.versions {
		// skip aliases btw
		if pkts_toml.version_aliases.contains_key(version) {
			continue;
		}
		let mut version_code = String::new();

		let mut directions: IndexMap<Direction, IndexMap<String, String>> = IndexMap::new();
		for (ty, all_version_groups) in &pkts_toml.types {
			match ty {
				Type::Common(_) => {
					for (&group_id, versions) in all_version_groups {
						if !versions.contains(version) {
							continue;
						}

						let type_name = ty.enum_name();
						let struct_name_with_version = ty.struct_name(group_id);
						version_code += &format!(
							"#[allow(unused_imports)] pub(crate) use super::{ty}::{group_id}::{struct_name_with_version} as {type_name};"
						);
					}
				}
				Type::Specific {
					direction,
					state,
					name: _,
				} => {
					for (&group_id, versions) in all_version_groups {
						if !versions.contains(version) {
							continue;
						}

						let specific_code = directions
							.entry(*direction)
							.or_default()
							.entry(state.0.clone())
							.or_default();

						let type_name = ty.enum_name();
						let struct_name_with_version = ty.struct_name(group_id);
						*specific_code += &format!(
							"#[allow(unused_imports)] pub(crate) use crate::types::{ty}::{group_id}::{struct_name_with_version} as {type_name};"
						);
					}
				}
			}
		}

		for direction in [Direction::S2C, Direction::C2S] {
			let mut direction_code = String::new();
			for state in ["handshaking", "login", "status", "configuration", "play"] {
				direction_code += &format!(
					"pub(crate) mod {state} {{ {} }}",
					directions
						.get(&direction)
						.map(|i| i.get(state))
						.flatten()
						.unwrap_or(&"".to_string())
				);
			}

			version_code += &format!("pub(crate) mod {direction} {{ {direction_code} }}");
		}

		code += &format!("pub(crate) mod v{version} {{ {version_code} }}");
	}

	format!("pub mod types {{ {code} }}")
}
