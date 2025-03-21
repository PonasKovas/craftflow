use crate::packets_toml::PacketsToml;

pub fn generate(pkts_toml: &PacketsToml) -> String {
	let mut arms = String::new();

	for (&direction, all_states) in &pkts_toml.packets {
		let mut inner_direction_p = String::new();
		let mut inner_direction_v = String::new();

		let direction_enum = direction.enum_name();
		for (state, all_packets) in all_states {
			let mut inner_state_p = String::new();
			let mut inner_state_v = String::new();

			let state_enum = state.enum_name();
			for (packet, all_version_groups) in all_packets {
				let mut inner_packet = String::new();

				let packet_enum = packet.enum_name();
				for (&version_group, _packet_ids) in all_version_groups {
					let version_variant = version_group.variant_name();

					inner_packet += &format!(
						"
						::craftflow_protocol::{direction}::{state}::{packet_enum}::{version_variant}($inner) => {{ $($code)+ }},\n"
					);
				}

				inner_state_p += &format!(
					"
					::craftflow_protocol::{direction}::{state_enum}::{packet_enum}($inner) => {{ $($code)+ }},\n"
				);
				inner_state_v += &format!(
					"
					::craftflow_protocol::{direction}::{state_enum}::{packet_enum}(inner) => {{ match inner {{
						{inner_packet}
						::craftflow_protocol::{direction}::{state}::{packet_enum}::_hidden(..) => unreachable!(),
					}} }},\n"
				);
			}

			inner_direction_p += &format!(
				"
				::craftflow_protocol::{direction_enum}::{state_enum}(inner) => match inner {{ {inner_state_p} }},\n"
			);
			inner_direction_v += &format!(
				"
				::craftflow_protocol::{direction_enum}::{state_enum}(inner) => match inner {{ {inner_state_v} }},\n"
			);
		}

		arms += &format!(
			"
			(({direction}->packet), $enum_value:ident -> $inner:ident $($code:tt)+) => {{ match $enum_value {{ {inner_direction_p} }} }};"
		);
		arms += &format!(
			"
			(({direction}->version), $enum_value:ident -> $inner:ident $($code:tt)+) => {{ match $enum_value {{ {inner_direction_v} }} }};"
		);
	}

	format!(
		r#"
	/// generates a HUGE match statement destructuring a given C2S or S2C packet enum and running some code
	/// on every single variant (both on packet enums and version structs)
	#[macro_export]
	#[doc(hidden)]
	macro_rules! enum_go_brr {{
		{arms}
	}}"#
	)
}
