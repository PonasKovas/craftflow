use crate::{
	gen_enum::{Variant, gen_enum},
	packets_toml::{Direction, PacketName, State, Version},
	shared::{closureslop_event_impl, versions_pattern},
};
use indexmap::IndexMap;

pub fn generate(
	direction: Direction,
	state: &State,
	packet: &PacketName,
	version_groups: &IndexMap<Version, IndexMap<u32, Vec<u32>>>,
) -> String {
	let dir_enum = direction.enum_name();
	let state_enum = state.enum_name();

	let enum_name = packet.enum_name();
	let enum_variants = version_groups
		.keys()
		.map(|&v| {
			let variant = v.variant_name();
			let pkt_path = format!("{packet}::{v}::{}", packet.struct_name(v));
			Variant {
				name: variant,
				value: pkt_path,
			}
		})
		// add an extra dochidden variant to encourage users to use the disabled_versions!() macro
		// (also it stops the unreachable pattern warning, since the pattern will always match this variant)
		// (and the macro cant add an attribute to disable the warning otherwise, bcs of how macros work)
		.chain([Variant {
			name: "#[allow(non_camel_case_types)] #[doc(hidden)] _hidden".to_string(),
			value: "".to_string(),
		}])
		.collect::<Vec<_>>();
	let enum_code = gen_enum(&enum_name, &enum_variants, true);

	let mut all_supported_versions = version_groups
		.values()
		.flat_map(|pkt_ids| pkt_ids.values().flatten())
		.map(ToString::to_string)
		.collect::<Vec<_>>();
	all_supported_versions.sort_unstable();
	all_supported_versions.dedup();
	let all_supported_versions_list: String = all_supported_versions.join(", ");
	let all_supported_versions_pattern: String = all_supported_versions.join("|");

	let write_match_arms: String = version_groups
		.keys()
		.map(|&group_id| {
			let pkt = group_id.variant_name();

			format!("Self::{pkt}(packet) => packet.packet_write(output, protocol_version),")
		})
		.collect();

	let read_match_arms: String = version_groups
		.iter()
		.map(|(&group_id, packet_ids)| {
			let inner_arms: String = packet_ids
				.iter()
				.map(|(&packet_id, versions)| {
					let pkt = packet.struct_name(group_id);
					let variant_name = group_id.variant_name();
					let pkt_id_versions_pattern = versions_pattern(versions);

					format!(
						"({packet_id}, {pkt_id_versions_pattern}) => Self::{variant_name}({packet}::{group_id}::{pkt}::mcp_read(input)?),"
					)
				})
				.collect();

			inner_arms
		})
		.collect();

	let closureslop_event_impl = closureslop_event_impl(&enum_name);

	format!(
		r#"{enum_code}

		impl crate::PacketWrite for {enum_name} {{
			fn packet_write(&self, output: &mut Vec<u8>, protocol_version: u32) -> usize {{
				match self {{
					{write_match_arms}
					Self::_hidden(..) => unreachable!(),
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
					(_, other) => return Err(Error::UnknownPacketId{{ id: other, protocol_version, state: "{direction}->{state}->{packet}" }}),
				}};

				Ok(packet)
			}}
		}}

		impl From<{enum_name}> for crate::{direction}::{state_enum} {{
			fn from(value: {enum_name}) -> Self {{
				Self::{enum_name}(value)
			}}
		}}
		impl From<{enum_name}> for crate::{dir_enum} {{
			fn from(value: {enum_name}) -> Self {{
				Self::{state_enum}(value.into())
			}}
		}}

		{closureslop_event_impl}
		"#,
	)
}
