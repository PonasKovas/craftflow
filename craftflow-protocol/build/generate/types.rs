use crate::{DEFAULT_IMPORTS_FOR_IMPLS, TYPES_DIR, packets_toml::PacketsToml, shared::package_dir};

pub fn generate(pkts_toml: &PacketsToml) -> String {
	let mut code = String::new();

	// first load the actual implementations in a clean-for-the-user way
	for (ty, all_version_groups) in &pkts_toml.types {
		let mut type_code = String::new();
		for (&group_id, _versions) in all_version_groups {
			let impl_path = package_dir()
				.join(TYPES_DIR)
				.join(ty.mod_name())
				.join(format!("{}.rs", group_id.mod_name()));
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

		code += &format!("pub mod {ty} {{ {type_code} }}");
	}

	// then generate private version modules and reexports for internal usage
	// for easy ALL types import for a specific version
	// use crate::types::vXXX::*;
	for version in &pkts_toml.versions {
		let mut version_code = String::new();

		for (ty, all_version_groups) in &pkts_toml.types {
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

		code += &format!("pub(crate) mod v{version} {{ {version_code} }}");
	}

	format!("pub mod types {{ {code} }}")
}
