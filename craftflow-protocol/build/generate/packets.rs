use crate::{PACKETS_DIR, packets_toml::PacketsToml, shared::package_dir};

mod direction_enum;
mod packet_enum;
mod state_enum;
mod version_group;

pub fn generate(pkts_toml: &PacketsToml) -> String {
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

					let version_group_code = version_group::generate(
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
				state_code += &packet_enum::generate(direction, state, packet, all_version_groups);

				state_code += &format!("pub mod {} {{ {packet_code} }}", packet.mod_name());
			}
			direction_code += &state_enum::generate(direction, state, &all_packets);

			direction_code += &format!("pub mod {} {{ {state_code} }}", state.mod_name());
		}
		code += &direction_enum::generate(direction, &all_states.keys().collect::<Vec<_>>());

		code += &format!("pub mod {} {{ {direction_code} }}", direction.mod_name());
	}

	code
}
