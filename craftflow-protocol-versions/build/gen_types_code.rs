use std::{
	fs::{self, read_dir},
	path::Path,
};

/// Reads the types/ directory and generates code to include it in the source tree
pub fn gen_types_code() -> String {
	let mut code = String::new();

	// First of all we actually have to read the directory
	let path = Path::new("types");
	for version_fs in read_dir(&path).unwrap().map(|f| f.unwrap()) {
		let version = version_fs.file_name().into_string().unwrap();

		let mut version_code = String::new();

		for type_fs in read_dir(&version_fs.path()).unwrap().map(|f| f.unwrap()) {
			let type_name = type_fs.file_name().into_string().unwrap();

			// there must be either a mod.rs for defined type
			// or a reexport files for reexports
			let mod_path = type_fs.path().join("mod.rs");
			if mod_path.exists() {
				let mod_path = mod_path.canonicalize().unwrap();
				version_code += &format!(
					r#"
				pub mod {type_name} {{
                    #[allow(unused_imports)]
                    use craftflow_protocol_core::datatypes::*;
                    #[allow(unused_imports)]
                    use craftflow_protocol_core::*;
                    #[allow(unused_imports)]
                    use std::borrow::Cow;
                    #[allow(unused_imports)]
                    use craftflow_protocol_core::common_structures::*;

				    include!({mod_path:?});
				}}
				"#,
				);
			} else {
				let reexported: u32 = fs::read_to_string(&type_fs.path().join("reexport"))
					.expect(&format!("reexport read {:?}", path))
					.trim()
					.parse()
					.expect(&format!("packet_reexport parse {:?}", path));

				version_code += &format!(
					r#"
				pub mod {type_name} {{ pub use crate::types::v{reexported:05}::{type_name}::*; }}
				"#,
				);
			}
		}

		code += &format!(
			r#"
			pub mod {version} {{
                {version_code}
            }}
		"#
		);
	}

	format!(
		r#"
	pub mod types {{
        {code}
	}}
	"#
	)
}
