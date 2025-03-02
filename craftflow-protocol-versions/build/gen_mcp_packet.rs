//! For generating PacketRead and PacketWrite for packet enums

use crate::parse_packet_info::{Direction, Generics, Packets, State};
use std::collections::HashMap;

pub fn gen_mcp_packet_impls(direction: &Direction, state: &State, packets: &Packets) -> String {
	let dir_mod = direction.mod_name();

	let generics = packets
		.iter()
		.fold(Generics::new(), |acc, (_, (generics, _))| {
			acc.union(generics)
		});
	let read_generics = Generics(vec!["'read".to_string(); generics.0.len()]).as_str();
	let generics = generics.as_str();
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

		let mut pattern_to_id = String::new();
		for (id, versions) in versions_by_id {
			let versions_pattern = versions
				.iter()
				.map(|v| v.0.to_string())
				.collect::<Vec<_>>()
				.join(" | ");

			read_match_arms += &format!(
				"({id}, {versions_pattern}) => {{
     			    let (input, packet) = MCPReadVersioned::read_versioned(input, protocol_version)
                        .with_context(|| format!(\"packet id {id}\"))?;
                    Ok((input, Self::{packet_variant}(packet)))
     			}},\n"
			);

			pattern_to_id += &format!("{} => {},\n", versions_pattern, id);
		}

		write_match_arms += &format!(
			"Self::{packet_variant}(packet) => {{
  		        let mut written = 0;
                let id = match protocol_version {{
                    {pattern_to_id}
                    other => return Err(Error::InvalidData(format!(\"{{other}} protocol version not supported by this packet\"))),
                }};
                written += VarInt(id).write(output)?;
                written += packet.write_versioned(output, protocol_version)?;
                Ok(written)
            }},\n"
		);
	}

	format!(
		r#"
		use craftflow_protocol_core::{{Result, Error, Context, datatypes::VarInt, MCPWrite, MCPRead}};
		use crate::{{MCPReadVersioned, MCPWriteVersioned, PacketRead, PacketWrite}};

		impl<'read> PacketRead<'read> for {path} {read_generics} {{
            fn read_packet(input: &'read [u8], protocol_version: u32) -> Result<(&'read [u8], Self)> {{
                let (input, packet_id) = VarInt::read(input)?;
                let packet_id = packet_id.0;
                match (packet_id, protocol_version) {{
                    {read_match_arms}
                    _ => Err(Error::InvalidData(format!("No packet found that has {{packet_id}} packet id on {{protocol_version}} protocol version"))),
                }}
            }}
        }}
        impl {generics} PacketWrite for {path} {generics} {{
            fn write_packet(&self, output: &mut impl std::io::Write, protocol_version: u32) -> Result<usize> {{
                match self {{
                    {write_match_arms}
                }}
            }}
        }}
    "#
	)
}
