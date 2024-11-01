//! For generating MCPReadVersioned and MCPWriteVersioned for version enums

use crate::{
	common::get_lifetime,
	parse_packet_info::{Direction, PacketName, PacketType, State, Version, Versions},
};
use std::collections::HashMap;

pub fn gen_mcp_versioned(
	direction: &Direction,
	state: &State,
	packet: &PacketName,
	versions: &Versions,
	lifetime: bool,
) -> String {
	let dir_mod = direction.mod_name();
	let state_mod = state.mod_name();
	let packet_mod = packet.mod_name();

	// iterate over versions and group them by re-exports
	let mut versions_grouped: HashMap<Version, Vec<Version>> = HashMap::new();
	for (version, info) in versions {
		match info.packet_type {
			PacketType::ReExport {
				version: reexported,
			} => {
				versions_grouped
					.entry(reexported)
					.or_insert_with(Vec::new)
					.push(*version);
			}
			PacketType::Defined { .. } => {
				versions_grouped
					.entry(*version)
					.or_insert_with(Vec::new)
					.push(*version);
			}
		}
	}

	// now iterate over the grouped versions and generate the match arms for read and write
	let mut read_match_arms = String::new();
	let mut write_match_arms = String::new();
	for (original, used_by) in versions_grouped {
		let version_variant = original.caps_mod_name();

		let versions_pattern = used_by
			.iter()
			.map(|v| v.0.to_string())
			.collect::<Vec<_>>()
			.join(" | ");

		read_match_arms += &format!(
			"{versions_pattern} => {{
		    let (input, packet) = MCPRead::read(input)?;
			Ok((input, Self::{version_variant}(packet)))
		}},\n"
		);

		write_match_arms += &format!(
		"Self::{version_variant}(packet) => {{
		    assert!(
				matches!(protocol_version, {versions_pattern}),
				\"Tried to write {packet_mod} packet with protocol version {{protocol_version}} but its only compatible with {versions_pattern}\"
			);
			packet.write(output)
		}},\n"
	);
	}

	let lifetime = get_lifetime(lifetime);
	let path = format!(
		"crate::{dir_mod}::{state_mod}::{pkt}",
		pkt = packet.enum_name(),
	);

	format!(
		r#"
		use craftflow_protocol_core::{{Error, Result, MCPRead, MCPWrite}};
		use crate::{{MCPReadVersioned, MCPWriteVersioned}};

        impl<'a> MCPReadVersioned<'a> for {path} {lifetime} {{
            fn read_versioned(input: &'a [u8], protocol_version: u32) -> Result<(&'a [u8], Self)> {{
                    match protocol_version {{
                        {read_match_arms}
                        _ => Err(Error::InvalidData(format!("This packet has no implementation for {{protocol_version}} protocol version"))),
                    }}
            }}
        }}
        impl {lifetime} MCPWriteVersioned for {path} {lifetime} {{
            fn write_versioned(&self, output: &mut impl std::io::Write, protocol_version: u32) -> Result<usize> {{
                match self {{
                    {write_match_arms}
                }}
            }}
        }}
    "#
	)
}
