//! For generating PacketRead and PacketWrite for packet enums

use std::collections::HashMap;

use crate::{
	common::get_lifetime,
	parse_packet_info::{Direction, PacketName, PacketType, Packets, State, Version, Versions},
};

pub fn gen_mcp_packet_impls(direction: &Direction, state: &State, packets: &Packets) -> String {
	let dir_mod = direction.mod_name();
	let state_mod = state.mod_name();

	let lifetime = get_lifetime(packets.iter().any(|(_, (has_lifetime, _))| *has_lifetime));
	let path = format!(
		"crate::{dir_mod}::{state_enum}",
		state_enum = state.enum_name(),
	);

	let mut read_match_arms = String::new();
	let mut write_match_arms = String::new();
	for (packet, (_, versions)) in packets {
		let packet_variant = packet.enum_name();

		// group the versions by packet_id for less code duplication and faster compile times
		let mut versions_by_id = HashMap::new();
		for (version, info) in versions {
			versions_by_id
				.entry(info.packet_id)
				.or_insert_with(Vec::new)
				.push(*version);
		}

		for (id, versions) in versions_by_id {
			let versions_pattern = versions
				.iter()
				.map(|v| v.0.to_string())
				.collect::<Vec<_>>()
				.join(" | ");

			read_match_arms += &format!(
				"({id}, {versions_pattern}) => {{
     			    let (input, packet) = crate::MCPReadVersioned::read_versioned(input, protocol_version)
                        .with_context(|| format!(\"packet id {id}, version {{protocol_version}}\"))?;
                    Ok((input, Self::{packet_variant}(packet)))
     			}},\n"
			);

			write_match_arms += &format!(
    			"{versions_pattern} => {{
    		        let mut written = 0;
                    written += VarInt({id}).write(output)?;
                    written += packet.write_versioned(output, protocol_version)
                        .with_context(|| format!(\"packet id {id}, version {{protocol_version}}\"))?;
                    Ok(written)
                }},\n"
    		);
		}
	}

	format!(
		r#"
		impl<'a> crate::PacketRead<'a> for {path} {lifetime} {{
            fn read_packet(input: &'a [u8], protocol_version: u32) -> craftflow_protocol_core::Result<(&'a [u8], Self)> {{
                let (input, packet_id) = craftflow_protocol_core::datatypes::VarInt::read(input)?;
                let packet_id = packet_id.0;
                match (packet_id, protocol_version) {{
                    {read_match_arms}
                    _ => Err(Error::InvalidData(format!("No packet found that has {{packet_id}} packet id on {{protocol_version}} protocol version"))),
                }}
            }}
        }}
        impl {lifetime} crate::PacketWrite for {path} {lifetime} {{
            fn write_packet(&self, output: &mut impl std::io::Write, protocol_version: u32) -> craftflow_protocol_core::Result<usize> {{
                match self {{
                    {write_match_arms}
                }}
            }}
        }}
    "#
	)
}
