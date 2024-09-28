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

fn get_state_packet_ids(path: impl AsRef<Path>) -> BTreeMap<String, u32> {
	let text = fs::read_to_string(path.as_ref()).expect(&format!("{:?}", path.as_ref()));

	let mut entries = BTreeMap::new();
	for (i, line) in text.split('\n').enumerate() {
		if line.is_empty() {
			continue;
		}

		let mut parts = line.splitn(2, ':');
		let packet_name = parts.next().expect(&format!("{:?}:{i}", path.as_ref()));
		let packet_id = parts
			.next()
			.expect(&format!("{:?}:{i}", path.as_ref()))
			.parse()
			.expect(&format!("{:?}:{i}", path.as_ref()));

		entries.insert(packet_name.to_owned(), packet_id);
	}

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

				let state_packet_ids = get_state_packet_ids(state.path().join("packet_ids"));

				let state_mod_name = state.file_name().into_string().unwrap();

				let mut packets = Vec::new();
				for packet in read_dir_sorted(state.path()) {
					if packet.file_name() == "mod.rs" || packet.file_name() == "packet_ids" {
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
				generate_state_enum(
					&version_mod_name,
					&direction,
					&state_mod_name,
					&packets,
					&state_packet_ids,
				);

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

fn generate_state_enum(
	version: &str,
	direction: &str,
	state: &str,
	packets: &[(String, String)],
	packet_ids: &BTreeMap<String, u32>,
) {
	let path = Path::new(&env::var("OUT_DIR").unwrap())
		.join(version)
		.join(direction);

	fs::create_dir_all(&path).unwrap();

	let enum_name = snake_to_pascal_case(state);

	let mut code = format!(
		"pub enum {enum_name} {{ {variants} }}",
		variants = packets
			.iter()
			.map(|(packet_mod_name, packet_name)| {
				format!("{packet_name}({state}::{packet_mod_name}::{packet_name}),\n")
			})
			.collect::<String>()
	);

	let gen_inner_write = || {
		let mut inner_write = String::new();
		for (packet_mod_name, packet_name) in packets {
			let packet_id = packet_ids[packet_mod_name];
			inner_write += &format!(
				"{packet_name}(packet) => {{
                    written += craftflow_protocol_core::VarInt({packet_id}).write(output)?;
                    written += packet.write(output)?;
                }}\n"
			);
		}
		inner_write
	};
	let gen_inner_read = || {
		let mut inner_read = String::new();
		for (packet_mod_name, packet_name) in packets {
			let packet_id = packet_ids[packet_mod_name];
			inner_read += &format!(
				"{packet_id} => {{
                    let (input, packet) = {state}::{packet_mod_name}::{packet_name}::read(input)?;
                    (input, {enum_name}::{packet_name}(packet))
                }}\n"
			);
		}
		inner_read
	};

	code += &format!(
		"
       	impl craftflow_protocol_core::MCPWrite for {enum_name} {{
       	    fn write(&self, output: &mut impl std::io::Write) -> craftflow_protocol_core::Result<usize> {{
     			let mut written = 0;

     			match self {{
                	{inner_write}
     			}}

      		    Ok(written)
      		}}
       	}}
        impl craftflow_protocol_core::MCPRead for {enum_name} {{
       	    fn read(input: &[u8]) -> craftflow_protocol_core::Result<(&[u8], Self)> {{
                let (input, packet_id) = craftflow_protocol_core::VarInt::read(input)?;

                let (input, packet) = match packet_id {{
                    {inner_read}
                    _ => return Err(craftflow_protocol_core::Error::InvalidData(format!(\"invalid packet id {{packet_id}}\"))),
                }};

                Ok((input, packet))
      		}}
       	}}
        ", inner_write = gen_inner_write(), inner_read = gen_inner_read(),
	);

	fs::write(path.join(state).with_extension("rs"), code).unwrap();
}

fn generate_direction_enum(version: &str, direction: &str, states: &[String]) {
	let path = Path::new(&env::var("OUT_DIR").unwrap()).join(version);

	fs::create_dir_all(&path).unwrap();

	let enum_name = direction.to_uppercase();

	let mut code = format!(
		"pub enum {enum_name} {{ {variants} }}",
		variants = states
			.iter()
			.map(|state| {
				format!(
					"{state_enum_name}({direction}::{state_enum_name}),\n",
					state_enum_name = snake_to_pascal_case(state)
				)
			})
			.collect::<String>()
	);

	code += &format!("
    	impl craftflow_protocol_core::MCPWrite for {enum_name} {{
       	    fn write(&self, output: &mut impl std::io::Write) -> craftflow_protocol_core::Result<usize> {{
     			match self {{
                   	{inner_write}
     			}}
      		}}
       	}}", inner_write = states
    		.iter()
    		.map(|state| {
    			format!(
    				"Self::{state_enum_name}(state) => state.write(output),\n",
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

	let enum_name = direction.to_uppercase();

	let mut code = format!(
		"pub enum {enum_name} {{ {variants} }}",
		variants = versions
			.iter()
			.map(|version| {
				format!(
					"{variant_name}({version}::{direction_enum_name}),\n",
					variant_name = version.to_uppercase(),
					direction_enum_name = direction.to_uppercase()
				)
			})
			.collect::<String>()
	);

	code += &format!("
    	impl craftflow_protocol_core::MCPWrite for {enum_name} {{
       	    fn write(&self, output: &mut impl std::io::Write) -> craftflow_protocol_core::Result<usize> {{
     			match self {{
                   	{inner_write}
     			}}
      		}}
       	}}", inner_write = versions
    		.iter()
    		.map(|version| {
    			format!(
    				"Self::{variant_name}(version) => version.write(output),\n",
    				variant_name = version.to_uppercase()
    			)
    		})
    		.collect::<String>()
	);

	fs::write(path, code).unwrap();
}
