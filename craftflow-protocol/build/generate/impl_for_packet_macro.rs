use crate::packets_toml::PacketsToml;

pub fn generate(pkts_toml: &PacketsToml) -> String {
	let mut direction_code = String::new();
	let mut state_code = String::new();
	let mut packet_code = String::new();
	let mut vpacket_code = String::new();
	for (&direction, all_states) in &pkts_toml.packets {
		direction_code += &format!(
			"const _: () = {{
				use craftflow_protocol::{} as direction; $code
			}};",
			direction.enum_name(),
		);
		for (state, all_packets) in all_states {
			state_code += &format!(
				"const _: () = {{
					use craftflow_protocol::{direction}::{} as state; $code
				}};",
				state.enum_name(),
			);
			for (packet, all_version_groups) in all_packets {
				packet_code += &format!(
					"const _: () = {{
						use craftflow_protocol::{direction}::{state}::{} as packet; $code
					}};",
					packet.enum_name(),
				);
				for (&version_group, _) in all_version_groups {
					vpacket_code += &format!(
						"const _: () = {{
							use craftflow_protocol::{direction}::{state}::{packet}::{version_group}::{} as vpacket; $code
						}};",
						packet.struct_name(version_group),
					);
				}
			}
		}
	}

	format!(
		"/// Expands the given implementation for all Directions/States/Packets/PacketVersions
	#[macro_export]
	macro_rules! impl_for {{
		(direction: $code:item) => {{ {direction_code} }};
		(state: $code:item) => {{ {state_code} }};
		(packet: $code:item) => {{ {packet_code} }};
		(vpacket: $code:item) => {{ {vpacket_code} }};
	}}"
	)
}
