use crate::{
	gen_enum::{Variant, gen_enum},
	packets_toml::{PacketName, Version},
	shared::versions_pattern,
};
use std::collections::HashMap;

pub fn generate(
	packet: &PacketName,
	version_groups: &HashMap<Version, HashMap<u32, Vec<u32>>>,
) -> String {
	let packet_enum_name = packet.enum_name();
	let enum_name = format!("{}Builder", packet.enum_name());
	let enum_variants = version_groups
		.keys()
		.map(|&v| {
			let name = v.variant_name();
			let version = packet.struct_name(v);
			let value = format!("crate::PacketEat<{packet}::{v}::{version}, {packet_enum_name}>");

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
	let enum_code = gen_enum(&enum_name, &enum_variants, false);

	let version_match_arms: String = version_groups
		.iter()
		.map(|(group_id, packet_ids)| {
			let pattern = versions_pattern(
				packet_ids
					.values()
					.flatten()
					.copied()
					.collect::<Vec<_>>()
					.as_slice(),
			);
			let variant = group_id.variant_name();

			format!("{pattern} => Self::{variant}(crate::PacketEat::new()),")
		})
		.collect();

	format!(
		r#"
		{enum_code}

		impl {enum_name} {{
			/// Constructs a new packet builder for a specific protocol version
			pub fn new(protocol_version: u32) -> Self {{
				match protocol_version {{
					{version_match_arms}
					other => panic!("Packet {enum_name} does not support {{other}} protocol version and cannot be built for it."),
				}}
			}}
		}}
		"#,
	)
}
