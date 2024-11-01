use crate::parse_packet_info::{Directions, PacketType};

pub fn gen_destructure_macro(directions: &Directions) -> String {
	let mut inner = String::new();

	for (direction, (_, states)) in directions {
		let mut inner_direction = String::new();

		let dir_mod = direction.mod_name();
		let dir_enum = direction.enum_name();
		for (state, (_, packets)) in states {
			let mut inner_state = String::new();

			let st_mod = state.mod_name();
			let st_enum = state.enum_name();
			for (packet, (_, versions)) in packets {
				let mut inner_packet = String::new();

				let pkt_enum = packet.enum_name();
				for (version, info) in versions {
					// skip re-exports
					if let PacketType::ReExport { .. } = info.packet_type {
						continue;
					}

					inner_packet += &format!(
                        "::craftflow_protocol_versions::{dir_mod}::{st_mod}::{pkt_enum}::{version_variant}($inner) => $code,\n",
                        version_variant = version.caps_mod_name(),
					);
				}
				inner_state += &format!(
                    "::craftflow_protocol_versions::{dir_mod}::{st_enum}::{pkt_enum}(inner) => match inner {{
                        {inner_packet}
                    }},\n",
                );
			}

			inner_direction += &format!(
				"::craftflow_protocol_versions::{dir_enum}::{st_enum}(inner) => match inner {{
                    {inner_state}
                }},\n",
			);
		}

		inner += &format!(
			"(direction={dir_enum}, $enum_value:ident -> $inner:ident $code:tt) => {{
                match $enum_value {{
                    {inner_direction}
                }}
            }};\n"
		);
	}

	format!(
		"// This macro is used internally in craftflow for the packet events.
		#[doc(hidden)]
		#[macro_export]
		macro_rules! __destructure_packet_enum__ {{
	       {inner}
		}}"
	)
}
