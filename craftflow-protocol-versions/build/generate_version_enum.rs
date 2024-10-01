use crate::common::snake_to_pascal_case;
use std::collections::BTreeMap;

pub fn generate_version_enum(
	direction: &str,
	state: &str,
	packet_name: &str,
	packet_versions: &BTreeMap<String, Vec<u32>>,
) -> String {
	let enum_name = snake_to_pascal_case(packet_name);
	let state_enum_name = snake_to_pascal_case(state);
	let direction_enum_name = direction.to_uppercase();

	let mut enum_variants = String::new();
	let mut packet_read_match_arms = String::new();
	let mut packet_write_match_arms = String::new();
	for (version_name, versions) in packet_versions {
		let variant_name = version_name.to_uppercase();
		let struct_name = format!("{}{}", enum_name, variant_name);

		let versions_comment = versions
			.iter()
			.map(|v| v.to_string())
			.collect::<Vec<_>>()
			.join(", ");

		enum_variants += &format!(
			"/// This variant applies for {versions_comment} protocol versions.
			{variant_name}({packet_name}::{version_name}::{struct_name}),\n"
		);

		let versions_pattern = versions
			.iter()
			.map(|v| v.to_string())
			.collect::<Vec<_>>()
			.join(" | ");
		packet_read_match_arms += &format!(
			"{versions_pattern} => {{
			    let (input, packet) = {packet_name}::{version_name}::{struct_name}::read(input)?;
				Ok((input, Self::{variant_name}(packet)))
			}},\n"
		);

		packet_write_match_arms += &format!(
			"Self::{variant_name}(packet) => {{
			    assert!(
					matches!(protocol_version, {versions_pattern}),
					\"Tried to write {packet_name} packet with protocol version {{protocol_version}} but its only compatible with {versions_comment}\"
				);
				packet.write(output)
			}},\n"
		);
	}

	format!(
		"pub use {packet_name}_enum::*;
		mod {packet_name}_enum {{
		use super::*;
		use craftflow_protocol_core::{{Result, MCPRead, MCPWrite, Error}};

		pub enum {enum_name} {{
                {enum_variants}
        }}

        impl crate::PacketRead for {enum_name} {{
            fn read_packet(input: &[u8], protocol_version: u32) -> Result<(&[u8], Self)> {{
                    match protocol_version {{
                        {packet_read_match_arms}
                        _ => Err(Error::InvalidData(format!(\"This packet has no implementation for {{protocol_version}} protocol version\"))),
                    }}
            }}
        }}
        impl crate::PacketWrite for {enum_name} {{
            fn write_packet(&self, output: &mut impl std::io::Write, protocol_version: u32) -> Result<usize> {{
                match self {{
                    {packet_write_match_arms}
                }}
            }}
        }}

        impl crate::IntoVersionEnum for {enum_name} {{
            type Packet = Self;

           	fn into_version_enum(self) -> Self::Packet {{
                self
            }}
        }}
        impl crate::IntoPacketEnum for {enum_name} {{
            type State = crate::{direction}::{state_enum_name};

            fn into_packet_enum(self) -> Self::State {{
                crate::{direction}::{state_enum_name}::{enum_name}(self)
            }}
        }}
        impl crate::IntoStateEnum for {enum_name} {{
            type Direction = crate::{direction_enum_name};

           	fn into_state_enum(self) -> Self::Direction {{
                crate::{direction_enum_name}::{state_enum_name}(crate::IntoPacketEnum::into_packet_enum(self))
            }}
        }}
        }}"
	)
}
