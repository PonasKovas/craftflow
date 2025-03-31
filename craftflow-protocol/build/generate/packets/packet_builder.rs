use crate::{
	gen_enum::{Variant, gen_enum},
	packets_toml::{PacketName, PacketsToml, Version},
	shared::{group_consecutive, versions_pattern},
};
use indexmap::IndexMap;

pub fn generate(
	pkts_toml: &PacketsToml,
	packet: &PacketName,
	version_groups: &IndexMap<Version, IndexMap<u32, Vec<u32>>>,
) -> String {
	let packet_enum_name = packet.enum_name();
	let enum_variants = version_groups
		.keys()
		.map(|&v| {
			let name = v.variant_name();
			let version = packet.struct_name(v);
			let value = format!("fn({packet}::{v}::{version}) -> {packet_enum_name}");

			Variant { name, value }
		})
		// add an extra dochidden variant to encourage users to use the disabled_versions!() macro
		// (also it stops the unreachable pattern warning, since the pattern will always match this variant)
		// (and the macro cant add an attribute to disable the warning otherwise, bcs of how macros work)
		.chain([Variant {
			name: "#[allow(non_camel_case_types)] #[doc(hidden)] _hidden".to_string(),
			value: "".to_string(),
		}])
		.collect::<Vec<_>>();
	let enum_name = format!("{}Builder", packet.enum_name(),);
	let enum_code = gen_enum(&enum_name, &enum_variants, false);

	let version_match_arms: String = version_groups
		.iter()
		.map(|(&group_id, packet_ids)| {
			let pattern = versions_pattern(
				packet_ids
					.values()
					.flatten()
					.copied()
					.collect::<Vec<_>>()
					.as_slice(),
			);
			let variant = group_id.variant_name();
			let version_struct = packet.struct_name(group_id);

			format!(
				"{pattern} => Self::{variant}({{
				fn _packet_eater(p: {packet}::{group_id}::{version_struct}) -> {packet_enum_name} {{
					p.into()
				}}
				_packet_eater
			}}),"
			)
		})
		.collect();

	let mut all_supported_versions = version_groups
		.values()
		.flat_map(|pkt_ids| pkt_ids.values().flatten())
		.copied()
		.collect::<Vec<_>>();
	all_supported_versions.sort_unstable();
	all_supported_versions.dedup();

	let all_supported_versions_pretty: String =
		group_consecutive(pkts_toml.versions.iter().map(|v| {
			(
				*v,
				all_supported_versions.contains(pkts_toml.version_aliases.get(v).unwrap_or(v)), // resolve alias if its an alias
			)
		}))
		.map(|(l, r, supported)| {
			let mark = if supported { '✅' } else { '❌' };
			format!("/// {mark} {l} - {r}\n///\n")
		})
		.collect::<String>();
	let all_supported_versions_str: Vec<String> = all_supported_versions
		.iter()
		.map(ToString::to_string)
		.collect();
	let all_supported_versions_len = all_supported_versions.len();
	let all_supported_versions_list: String = all_supported_versions_str.join(", ");

	format!(
		r#"
		/// This packet is used in the following protocol versions:
		///
		{all_supported_versions_pretty}
		{enum_code}

		impl {enum_name} {{
			pub const VERSIONS: [u32; {all_supported_versions_len}] = [{all_supported_versions_list}];
		}}


		impl {enum_name} {{
			/// Constructs a new packet builder for a specific protocol version
			pub fn new(protocol_version: u32) -> Self {{
				match protocol_version {{
					{version_match_arms}
					other => panic!("Packet {enum_name} does not support {{other}} protocol version and cannot be built for it."),
				}}
			}}
		}}

		impl crate::PacketBuilder for {enum_name} {{
			type Packet = {packet_enum_name};

			const VERSIONS: &'static [u32] = &Self::VERSIONS;

			fn new(protocol_version: u32) -> Self {{
				Self::new(protocol_version)
			}}
		}}
		"#,
	)
}
