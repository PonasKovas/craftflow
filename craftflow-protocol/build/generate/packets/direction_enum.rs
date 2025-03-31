use crate::{
	gen_enum::{Variant, gen_enum},
	packets_toml::{Direction, State},
};

pub fn generate(direction: Direction, all_states: &[&State]) -> String {
	let enum_name = direction.enum_name();
	let enum_variants = all_states
		.iter()
		.map(|state| {
			let state_enum = state.enum_name();
			let state_path = format!("{direction}::{state_enum}");
			Variant {
				name: state_enum,
				value: state_path,
			}
		})
		.collect::<Vec<_>>();
	let enum_code = gen_enum(enum_name, &enum_variants, true);

	let write_match_arms: String = all_states
		.iter()
		.map(|state| {
			let state = state.enum_name();

			format!("Self::{state}(state) => state.packet_write(output, protocol_version),")
		})
		.collect();

	format!(
		r#"{enum_code}

		impl crate::PacketWrite for {enum_name} {{
			fn packet_write(&self, output: &mut Vec<u8>, protocol_version: u32) -> usize {{
				match self {{
					{write_match_arms}
				}}
			}}
		}}

		
		"#,
	)
}
