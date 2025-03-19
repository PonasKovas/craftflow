use crate::{
	gen_enum::{gen_enum, Variant}, packets_toml::{Direction, PacketName, State, Version}, shared::versions_pattern, DEFAULT_IMPORTS_FOR_IMPLS
};
use std::collections::HashMap;

pub fn generate(
	direction: Direction,
	state: &State,
	all_packets: &HashMap<PacketName, HashMap<Version, HashMap<u32, Vec<u32>>>>,
) -> String {
	let dir_enum = direction.enum_name();

	let enum_name = state.enum_name();
	let enum_variants = all_packets
		.keys()
		.map(|packet| {
			let pkt = packet.enum_name();
			let pkt_path = format!("{state}::{pkt}");
			Variant {
				name: pkt,
				value: pkt_path,
			}
		})
		.collect::<Vec<_>>();
	let enum_code = gen_enum(&enum_name, &enum_variants);

	let all_supported_versions = all_packets
		.values()
		.map(|version_groups| version_groups.values().map(|pkt_ids| pkt_ids.values()))
		.flatten()
		.flatten()
		.flatten()
		.map(ToString::to_string)
		.collect::<Vec<_>>();
	let all_supported_versions_list: String = all_supported_versions.join(", ");
	let all_supported_versions_pattern: String = all_supported_versions.join("|");

	let write_match_arms: String = all_packets
		.keys()
		.map(|packet| {
			let pkt = packet.enum_name();

			format!("Self::{pkt}(packet) => packet.packet_write(output, protocol_version),")
		})
		.collect();

	let read_match_arms: String = all_packets.iter().map(|(packet, version_groups)| 
		version_groups
		.iter()
		.map(|(&group_id, packet_ids)| 
			packet_ids
				.iter()
				.map(|(&packet_id, versions)| {
					let packet_enum = packet.enum_name();
					let packet_struct = packet.struct_name( group_id);
					let versions_pattern = versions_pattern(versions);

					format!(
						"({packet_id}, {versions_pattern}) => Self::{packet_enum}({state}::{packet_enum}::{packet_struct}(
							<{state}::{packet}::{group_id}::{packet_struct} as crate::MCPRead>::mcp_read(input)?
						)),"
					)
				}).collect::<String>()
		).collect::<String>()
	).collect();

	format!(
		r#"{DEFAULT_IMPORTS_FOR_IMPLS}
		{enum_code}

		impl crate::PacketWrite for {enum_name} {{
			fn packet_write(&self, output: &mut Vec<u8>, protocol_version: u32) -> usize {{
				match self {{
					{write_match_arms}
				}}
			}}
		}}
		impl<'a> crate::PacketRead<'a> for {enum_name} {{
			fn packet_read(input: &mut &'a [u8], protocol_version: u32) -> Result<Self> {{
				if !matches!(protocol_version, {all_supported_versions_pattern}) {{
					panic!("{enum_name} cannot be read in {{protocol_version}} protocol version. Supported versions: {all_supported_versions_list}");
				}}
				let packet_id = <crate::datatypes::VarInt as crate::MCPRead>::mcp_read(input)?.0 as u32;
				let packet = match (packet_id, protocol_version) {{
					{read_match_arms}
					(other, _) => return Err(Error::UnknownPacketId{{ id: other, protocol_version, state: "{direction}->{state}" }}),
				}};

				Ok(packet)
			}}
		}}

		impl From<{enum_name}> for crate::{dir_enum} {{
			fn from(value: {enum_name}) -> Self {{
				Self::{enum_name}(value)
			}}
		}}
		"#,
	)
}
