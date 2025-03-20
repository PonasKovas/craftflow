use crate::packets_toml::PacketsToml;
use std::env;

pub fn generate(pkts_toml: &PacketsToml) -> String {
	let mut arms = String::new();

	for (&direction, all_states) in &pkts_toml.packets {
		for (state, all_packets) in all_states {
			for (packet, all_version_groups) in all_packets {
				let packet_name = packet.enum_name();

				let mut packet_patterns = format!(
					"::craftflow_protocol::{direction}::{state}::{packet_name}::_hidden(..)"
				);
				let mut builder_patterns = format!(
					"::craftflow_protocol::{direction}::{state}::{packet_name}Builder::_hidden(..)"
				);
				for (&version_group, packet_ids) in all_version_groups {
					// if all versions of this version group are disabled, add it to the pattern
					let all_versions = packet_ids.values().flat_map(|v| v.iter()).copied();
					let all_disabled = all_versions.fold(true, |acc, version| {
						acc && env::var(format!("CARGO_FEATURE_NO_V{}", version)).is_ok()
					});

					if all_disabled {
						let variant = version_group.variant_name();
						packet_patterns += &format!(
							" | ::craftflow_protocol::{direction}::{state}::{packet_name}::{variant}(..)"
						);
						builder_patterns += &format!(
							" | ::craftflow_protocol::{direction}::{state}::{packet_name}Builder::{variant}(..)"
						);
					}
				}

				arms += &format!(
					r#"
					({direction}::{state}::{packet_name}) => {{ {packet_patterns} }};
					({direction}::{state}::{packet_name}Builder) => {{ {builder_patterns} }};
				"#
				);
			}
		}
	}

	format!(
		r#"
	/// Generates a pattern for disabled protocol versions when matching a packet.
	#[macro_export]
	macro_rules! disabled_versions {{
		{arms}
	}}"#
	)
}
