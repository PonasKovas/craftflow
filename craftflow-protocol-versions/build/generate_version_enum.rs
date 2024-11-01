//! Generates an enum for a packet containing all version variants of it, also conversion trait impls for the enum
//! and for every single version variant itself.

use crate::{
	common::snake_to_pascal_case,
	parse_packet_info::{Direction, HasLifetime, PacketName, State, Version},
};

pub fn generate_version_enum(
	direction: Direction,
	state: &State,
	packet: &PacketName,
	packet_versions: &[(Version, HasLifetime)],
) -> String {
	let mut enum_variants = String::new();
	let mut packet_read_match_arms = String::new();
	let mut packet_write_match_arms = String::new();
	for (version, has_lifetime) in packet_versions {
		let versions_comment = versions
			.iter()
			.map(|(v, _id)| v.to_string())
			.collect::<Vec<_>>()
			.join(", ");

		enum_variants += &format!(
			"/// This variant applies for {versions_comment} protocol versions.
			{variant_name}({packet_name}::{version_name}::{struct_name}<'a>),\n"
		);

		let versions_pattern = versions
			.iter()
			.map(|(v, _id)| v.to_string())
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

		#[derive(Debug, PartialEq, Clone)]
		pub enum {enum_name}<'a> {{
            {enum_variants}
        }}

        impl<'a> crate::MCPReadVersioned<'a> for {enum_name}<'a> {{
            fn read_versioned(input: &'a [u8], protocol_version: u32) -> Result<(&'a [u8], Self)> {{
                    match protocol_version {{
                        {packet_read_match_arms}
                        _ => Err(Error::InvalidData(format!(\"This packet has no implementation for {{protocol_version}} protocol version\"))),
                    }}
            }}
        }}
        impl<'a> crate::MCPWriteVersioned for {enum_name}<'a> {{
            fn write_versioned(&self, output: &mut impl std::io::Write, protocol_version: u32) -> Result<usize> {{
                match self {{
                    {packet_write_match_arms}
                }}
            }}
        }}

        impl<'a> crate::IntoVersionEnum for {enum_name}<'a> {{
            type Packet = Self;

           	fn into_version_enum(self) -> Self::Packet {{
                self
            }}
        }}
        impl<'a> crate::IntoPacketEnum for {enum_name}<'a> {{
            type State = crate::{direction}::{state_enum_name}<'a>;

            fn into_packet_enum(self) -> Self::State {{
                crate::{direction}::{state_enum_name}::{enum_name}(self)
            }}
        }}
        impl<'a> crate::IntoStateEnum for {enum_name}<'a> {{
            type Direction = crate::{direction_enum_name}<'a>;

           	fn into_state_enum(self) -> Self::Direction {{
                crate::{direction_enum_name}::{state_enum_name}(crate::IntoPacketEnum::into_packet_enum(self))
            }}
        }}
        }}"
	)
}
