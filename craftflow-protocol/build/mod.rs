use shared::{out_dir, package_dir};
use std::{collections::HashMap, env, fs, process::Command};

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
	for (&direction, all_states) in &pkts_toml.packets {
		let mut direction_code = String::new();
		for (state, all_packets) in all_states {
			let mut state_code = String::new();
			for (packet, all_version_groups) in all_packets {
				let mut packet_code = String::new();
				for (&version_group, packet_ids) in all_version_groups {
					let impl_path = package_dir()
						.join(PACKETS_DIR)
						.join(direction.mod_name())
						.join(&state.0)
						.join(&packet.0)
						.join(format!("{}.rs", version_group.mod_name()));

					let version_group_code = generate::version_group(
						direction,
						state,
						packet,
						version_group,
						packet_ids,
						impl_path.to_str().expect("impl path not valid utf8"),
					);

					packet_code += &format!(
						"pub mod {} {{ {version_group_code} }}",
						version_group.mod_name()
					);
				}
				state_code += &generate::packet_enum(direction, state, packet, all_version_groups);

				state_code += &format!("pub mod {} {{ {packet_code} }}", packet.mod_name());
			}
			direction_code += &generate::state_enum(direction, state, &all_packets);

			direction_code += &format!("pub mod {} {{ {state_code} }}", state.mod_name());
		}
		code += &format!("pub mod {} {{ {direction_code} }}", direction.mod_name());
	}

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
