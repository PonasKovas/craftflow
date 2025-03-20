use shared::{out_dir, package_dir};
use std::{env, fs, process::Command};

mod gen_enum;
mod generate;
mod packets_toml;
mod shared;

const PACKETS_TOML: &str = "packets.toml";
const PROMPT_CODE_EXAMPLE_PATH: &str = "generator/gen/example_code.rs";
const PACKETS_DIR: &str = "packets/";
const GENERATED_CODE_PATH: &str = "generated.rs";
const DEFAULT_ENUM_DERIVES: &str = "#[derive(Debug, PartialEq, Clone)]";
const DEFAULT_IMPORTS_FOR_IMPLS: &str = "use crate::datatypes::*;
use crate::{Error, MCPRead, MCPWrite, Result};";

fn main() {
	let pkts_toml = packets_toml::load();

	let mut code = String::new();

	// Generate packets and their enums
	code += &generate::packets(&pkts_toml);

	code += &generate::packet_builders(&pkts_toml);

	// disabled_versions!() macro
	code += &generate::disabled_versions_macro(&pkts_toml);

	// also include the prompt example test to be compiled the same way a normal packet impl would be to make sure
	// its not outdated or anything - we dont want to confuse the LLM for no reason.
	let prompt_code_example_path = package_dir().join(PROMPT_CODE_EXAMPLE_PATH);
	let prompt_code_example_path = prompt_code_example_path
		.to_str()
		.expect("impl path not valid utf8");
	code += &format!(
		r#"const _: () = {{
		mod prompt_example_code {{ {DEFAULT_IMPORTS_FOR_IMPLS} include!{{ "{prompt_code_example_path}" }} }}
	}};"#
	);

	// Write all the generated code
	fs::write(out_dir().join(GENERATED_CODE_PATH), code).expect("writing generated code");

	// Rustfmt it also
	if env::var("NO_FMT").is_err() {
		let rustfmt = env::var("RUSTFMT").unwrap_or("rustfmt".to_string());

		Command::new(rustfmt)
			.arg("--edition")
			.arg("2024")
			.arg(out_dir().join(GENERATED_CODE_PATH))
			.status()
			.expect("rustfmt failed");
	}
}
