use crate::{
	DEFAULT_IMPORTS_FOR_IMPLS,
	gen_enum::{Variant, gen_enum},
	packets_toml::{Direction, PacketName, State, Version},
	shared::versions_pattern,
};
use std::collections::HashMap;

pub fn generate(
	direction: Direction,
	state: &State,
	packet: &PacketName,
	version_groups: &HashMap<Version, HashMap<u32, Vec<u32>>>,
) -> String {
	let dir_enum = direction.mod_name();
	let state_enum = state.enum_name();
	let enum_name = packet.enum_name();
	let enum_variants = version_groups
		.keys()
		.map(|&v| {
			let pkt = packet.struct_name(v);
			let pkt_path = format!("{packet}::{v}::{pkt}");
			Variant {
				name: pkt,
				value: pkt_path,
			}
		})
		.collect::<Vec<_>>();
	let enum_code = gen_enum(&enum_name, &enum_variants);

	let all_supported_versions: String = version_groups
		.values()
		.map(|pkt_ids| pkt_ids.values().flatten())
		.flatten()
		.map(ToString::to_string)
		.collect::<Vec<_>>()
		.join(", ");

	let write_match_arms: String = version_groups
		.keys()
		.map(|&group_id| {
			let pkt = packet.struct_name(group_id);

			format!("Self::{pkt}(packet) => packet.packet_write(output, protocol_version),")
		})
		.collect();

	let read_match_arms: String = version_groups
		.iter()
		.map(|(&group_id, packet_ids)| {
			let all_group_versions =
				versions_pattern(&packet_ids.values().flatten().copied().collect::<Vec<_>>());

			let inner_arms: String = packet_ids
				.iter()
				.map(|(&packet_id, versions)| {
					let pkt = packet.struct_name(group_id);
					let pkt_id_versions_pattern = versions_pattern(versions);

					format!(
						"({packet_id}, {pkt_id_versions_pattern}) => Self::{pkt}({packet}::{group_id}::{pkt}::mcp_read(input)?),"
					)
				})
				.collect();

			format!(r#"{all_group_versions} => match (packet_id, protocol_version) {{
				{inner_arms}
				(other, _) => return Err(Error::UnknownPacketId{{ id: other, protocol_version, state: "{direction}->{state}->{packet}" }}),
			}},"#)
		})
		.collect();

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
				let packet_id = <crate::datatypes::VarInt as crate::MCPRead>::mcp_read(input)?.0 as u32;
				let packet = match protocol_version {{
					{read_match_arms}
					other => panic!("{enum_name} cannot be read in {{other}} protocol version. Supported versions: {all_supported_versions}"),
				}};

				Ok(packet)
			}}
		}}

		impl From<{enum_name}> for crate::{direction}::{state_enum} {{
			fn from(value: {enum_name}) -> Self {{
				Self::{enum_name}(value)
			}}
		}}
		"#,
	)
}
