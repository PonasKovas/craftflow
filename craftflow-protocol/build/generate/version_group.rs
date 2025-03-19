use crate::{
	DEFAULT_IMPORTS_FOR_IMPLS,
	packets_toml::{Direction, PacketName, State, Version},
	shared::versions_pattern,
};
use std::collections::HashMap;

pub fn generate(
	direction: Direction,
	state: &State,
	packet: &PacketName,
	version_group: Version,
	packet_ids: &HashMap<u32, Vec<u32>>,
	impl_path: &str,
) -> String {
	let dir_enum = direction.enum_name();
	let state_enum = state.enum_name();
	let packet_enum = packet.enum_name();

	let struct_name = packet.struct_name(version_group);

	let all_supported_versions: String = packet_ids
		.iter()
		.map(|(_, versions)| versions)
		.flatten()
		.map(ToString::to_string)
		.collect::<Vec<_>>()
		.join(", ");

	let write_match_arms: String = packet_ids
		.iter()
		.map(|(&id, versions)| {
			let pattern = versions_pattern(versions);

			format!("{pattern} => {id},")
		})
		.collect();

	let read_match_arms: String = packet_ids
		.iter()
		.map(|(&id, versions)| {
			let pattern = versions_pattern(versions);

			format!("{pattern} => {id},")
		})
		.collect();

	format!(
		r#"{DEFAULT_IMPORTS_FOR_IMPLS}
		include!{{ "{impl_path}" }}

		impl crate::PacketWrite for {struct_name} {{
			fn packet_write(&self, output: &mut Vec<u8>, protocol_version: u32) -> usize {{
				let id = match protocol_version {{
					{write_match_arms}
					other => panic!("{struct_name} cannot be written in {{other}} protocol version. Supported versions: {all_supported_versions}"),
				}};
				crate::MCPWrite::mcp_write(&crate::datatypes::VarInt(id), output) +
				crate::MCPWrite::mcp_write(self, output)
			}}
		}}
		impl<'a> crate::PacketRead<'a> for {struct_name} {{
			fn packet_read(input: &mut &'a [u8], protocol_version: u32) -> Result<Self> {{
				let packet_id = <crate::datatypes::VarInt as crate::MCPRead>::mcp_read(input)?.0 as u32;
				let expected_packet_id = match protocol_version {{
					{read_match_arms}
					other => panic!("{struct_name} cannot be read in {{other}} protocol version. Supported versions: {all_supported_versions}"),
				}};
				if packet_id != expected_packet_id {{
					return Err(Error::WrongPacketId {{ found: packet_id, expected: expected_packet_id }});
				}}
				crate::MCPRead::mcp_read(input)
			}}
		}}

		impl From<{struct_name}> for crate::{direction}::{state}::{packet_enum} {{
			fn from(value: {struct_name}) -> Self {{
				Self::{struct_name}(value)
			}}
		}}
		impl From<{struct_name}> for crate::{direction}::{state_enum} {{
			fn from(value: {struct_name}) -> Self {{
				Self::{packet_enum}(value.into())
			}}
		}}
		impl From<{struct_name}> for crate::{dir_enum} {{
			fn from(value: {struct_name}) -> Self {{
				Self::{state_enum}(crate::{direction}::{state_enum}::{packet_enum}(value.into()))
			}}
		}}
		"#,
	)
}
