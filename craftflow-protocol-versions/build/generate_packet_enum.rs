use crate::common::snake_to_pascal_case;
use std::collections::BTreeMap;

pub fn generate_packet_enum(
	direction: &str,
	state: &str,
	state_packets: &BTreeMap<String, BTreeMap<String, (Vec<u32>, u32)>>,
) -> String {
	let enum_name = snake_to_pascal_case(state);
	let direction_enum_name = direction.to_uppercase();

	let mut enum_variants = String::new();
	let mut packet_read_match_arms = String::new();
	let mut packet_write_match_arms = String::new();

	for (packet, version_to_id) in state_packets {
		let variant_name = snake_to_pascal_case(packet);

		enum_variants += &format!("{variant_name}({state}::{variant_name}),\n");

		let mut inner_write_match_arms = String::new();

		for (_version_variant, (versions, packet_id)) in version_to_id {
			let versions_pattern = versions
				.iter()
				.map(|v| v.to_string())
				.collect::<Vec<_>>()
				.join(" | ");
			let first_version = versions[0];

			packet_read_match_arms += &format!(
				"({versions_pattern}, {packet_id}) => {{
    			    let (input, packet) = {state}::{variant_name}::read_packet(input, {first_version})?;
    				Ok((input, Self::{variant_name}(packet)))
    			}},\n"
			);

			inner_write_match_arms += &format!(
				"{versions_pattern} => {{
				    let mut written = 0;
                    written += VarInt({packet_id}).write(output)?;
                    written += packet.write_packet(output, {first_version})?;
                    Ok(written)
                }},\n"
			);
		}

		packet_write_match_arms += &format!(
			"Self::{variant_name}(packet) => match protocol_version {{
			    {inner_write_match_arms}
				_ => Err(Error::InvalidData(
				    format!(
						\"Packet {variant_name} can't be written on {{protocol_version}} protocol version\",
					)
				)),
			}},\n"
		);
	}

	format!(
		"pub use {state}_enum::*;
		mod {state}_enum {{
		use super::*;
		use craftflow_protocol_core::{{Result, MCPRead, MCPWrite, Error, datatypes::VarInt}};
		use crate::{{PacketRead, PacketWrite}};

		#[derive(Debug, PartialEq, Clone, Hash)]
		pub enum {enum_name} {{
            {enum_variants}
        }}

        impl PacketRead for {enum_name} {{
            fn read_packet(input: &mut [u8], protocol_version: u32) -> Result<(&mut [u8], Self)> {{
                let (input, packet_id) = VarInt::read(input)?;
                let packet_id = packet_id.0;
                match (protocol_version, packet_id) {{
                    {packet_read_match_arms}
                    _ => Err(Error::InvalidData(format!(\"No packet found that has {{packet_id}} packet id on {{protocol_version}} protocol version\"))),
                }}
            }}
        }}
        impl PacketWrite for {enum_name} {{
            fn write_packet(&self, output: &mut impl std::io::Write, protocol_version: u32) -> Result<usize> {{
                match self {{
                    {packet_write_match_arms}
                }}
            }}
        }}

        impl crate::IntoPacketEnum for {enum_name} {{
            type State = Self;

            fn into_packet_enum(self) -> Self::State {{
                self
            }}
        }}
        impl crate::IntoStateEnum for {enum_name} {{
            type Direction = crate::{direction_enum_name};

           	fn into_state_enum(self) -> Self::Direction {{
                super::super::{direction_enum_name}::{enum_name}(self)
            }}
        }}
        }}"
	)
}
