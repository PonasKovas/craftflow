// This build script generates packet enums for every version and state.

use std::{
	collections::BTreeMap,
	env,
	fs::{self, read_dir},
	path::Path,
};

fn snake_to_pascal_case(s: &str) -> String {
	let mut result = String::new();
	let mut capitalize = true;
	for c in s.chars() {
		if c == '_' {
			capitalize = true;
		} else {
			if capitalize {
				result.push(c.to_ascii_uppercase());
				capitalize = false;
			} else {
				result.push(c);
			}
		}
	}
	result
}

fn read_dir_sorted(path: impl AsRef<Path>) -> Vec<std::fs::DirEntry> {
	let mut entries = read_dir(path)
		.unwrap()
		.map(|d| d.unwrap())
		.collect::<Vec<_>>();
	entries.sort_by_key(|entry| entry.path());
	entries
}

fn main() {
	// direction -> Vec<Version>
	let mut versions = BTreeMap::new();

	for direction in ["c2s", "s2c"] {
		versions.insert(direction, Vec::new());

		for version in read_dir_sorted("src/") {
			if !version.file_type().unwrap().is_dir() {
				continue;
			}
			let version_mod_name = version.file_name().into_string().unwrap();

			let direction_path = version.path().join(direction);
			if !direction_path.exists() {
				continue;
			}

			let mut states = Vec::new();
			for state in read_dir_sorted(direction_path) {
				if !state.file_type().unwrap().is_dir() {
					continue;
				}

				let state_mod_name = state.file_name().into_string().unwrap();

				let mut packets = Vec::new();
				for packet in read_dir_sorted(state.path()) {
					if packet.file_name() == "mod.rs" {
						continue;
					}

					let packet_mod_name = packet
						.path()
						.file_stem()
						.unwrap()
						.to_owned()
						.into_string()
						.unwrap();
					let packet_name = snake_to_pascal_case(&packet_mod_name);

					packets.push((packet_mod_name, packet_name));
				}
				generate_state_enum(&version_mod_name, &direction, &state_mod_name, &packets);

				states.push(state_mod_name);
			}

			generate_direction_enum(&version_mod_name, &direction, &states);

			versions.get_mut(direction).unwrap().push(version_mod_name);
		}
	}

	for direction in ["c2s", "s2c"] {
		generate_version_enum(direction, &versions[direction]);
	}
}

fn generate_state_enum(version: &str, direction: &str, state: &str, packets: &[(String, String)]) {
	let path = Path::new(&env::var("OUT_DIR").unwrap())
		.join(version)
		.join(direction);

	fs::create_dir_all(&path).unwrap();

	let code = format!(
		"pub enum {enum_name} {{ {variants} }}",
		enum_name = snake_to_pascal_case(state),
		variants = packets
			.iter()
			.map(|(packet_mod_name, packet_name)| {
				format!("{packet_name}({state}::{packet_mod_name}::{packet_name}),")
			})
			.collect::<String>()
	);

	fs::write(path.join(state).with_extension("rs"), code).unwrap();
}

fn generate_direction_enum(version: &str, direction: &str, states: &[String]) {
	let path = Path::new(&env::var("OUT_DIR").unwrap()).join(version);

	fs::create_dir_all(&path).unwrap();

	let code = format!(
		"pub enum {enum_name} {{ {variants} }}",
		enum_name = direction.to_uppercase(),
		variants = states
			.iter()
			.map(|state| {
				format!(
					"{state}({direction}::{state_enum_name}),",
					state_enum_name = snake_to_pascal_case(state)
				)
			})
			.collect::<String>()
	);

	fs::write(path.join(direction).with_extension("rs"), code).unwrap();
}

fn generate_version_enum(direction: &str, versions: &[String]) {
	let path = Path::new(&env::var("OUT_DIR").unwrap())
		.join(direction)
		.with_extension("rs");

	let code = format!(
		"pub enum {enum_name} {{ {variants} }}",
		enum_name = direction.to_uppercase(),
		variants = versions
			.iter()
			.map(|version| {
				format!(
					"{variant_name}({version}::{direction_enum_name}),",
					variant_name = version.to_uppercase(),
					direction_enum_name = direction.to_uppercase()
				)
			})
			.collect::<String>()
	);

	fs::write(path, code).unwrap();
}
