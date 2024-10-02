use crate::common::{read_dir_sorted, snake_to_pascal_case};
use std::path::Path;

pub fn gen_destructure_macro() -> String {
	let mut inner = String::new();

	for direction in ["c2s", "s2c"] {
		let mut inner_direction = String::new();

		let dir = direction.to_uppercase();
		let direction_path = Path::new("src/").join(direction);
		if direction_path.exists() {
			for state in read_dir_sorted(&direction_path) {
				if state.file_type().unwrap().is_dir() {
					let mut inner_state = String::new();

					let state_name = state.file_name().into_string().unwrap();
					let state_variant = snake_to_pascal_case(&state_name);
					for packet in read_dir_sorted(&state.path()) {
						if packet.file_type().unwrap().is_dir() {
							let mut inner_packet = String::new();

							let packet_name = packet.file_name().into_string().unwrap();
							let packet_variant = snake_to_pascal_case(&packet_name);
							for version in read_dir_sorted(&packet.path()) {
								if !version.file_type().unwrap().is_dir() {
									continue;
								}

								let version_name = version.file_name().into_string().unwrap();
								let version_variant = snake_to_pascal_case(&version_name);

								inner_packet += &format!(
                                    "::craftflow_protocol_versions::{dir}::{state_name}::{packet_variant}::{version_variant}(inner) => $code,\n",
                                );
							}
							inner_state += &format!(
                                "::craftflow_protocol_versions::{direction}::{state_variant}::{packet_variant}(inner) => match inner {{
                                    {inner_packet}
                                }},\n",
                            );
						}
					}

					inner_direction += &format!(
						"::craftflow_protocol_versions::{dir}::{state_variant}(inner) => match inner {{
                        {inner_state}
                    }},\n",
					);
				}
			}
		}

		inner += &format!(
			"(direction={dir}, $enum_value:ident -> $code:tt) => {{
                match $enum_value {{
                    {inner_direction}
                }}
            }};\n"
		);
	}

	format!(
		"// This macro is used internally in craftflow for the packet events.
		#[doc(hidden)]
		#[macro_export]
		macro_rules! __destructure_packet_enum__ {{
	       {inner}
		}}"
	)
}
