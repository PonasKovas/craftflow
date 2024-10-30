use crate::{
	common::{read_dir_sorted, snake_to_pascal_case},
	parse_packet_info::parse_packet_info,
};
use std::path::Path;

pub fn gen_impl_trait_macro() -> String {
	let mut inner = String::new();
	let mut inner_post = String::new();

	for direction in ["c2s", "s2c"] {
		let direction_path = Path::new("src/").join(direction);
		if direction_path.exists() {
			for state in read_dir_sorted(&direction_path) {
				if state.file_type().unwrap().is_dir() {
					let state_name = state.file_name().into_string().unwrap();
					for packet in read_dir_sorted(&state.path()) {
						if packet.file_type().unwrap().is_dir() {
							let packet_name = packet.file_name().into_string().unwrap();
							for version in read_dir_sorted(&packet.path()) {
								if !version.file_type().unwrap().is_dir() {
									continue;
								}

								// check if this is not a re-export
								let packet_info = parse_packet_info(version.path());
								if packet_info.reexport.is_some() {
									continue;
								}

								let version_name = version.file_name().into_string().unwrap();

								inner += &format!("const _: () = {{ type X<'a> = ::craftflow_protocol_versions::{direction}::{state_name}::{packet_name}::{version_name}::{pkt_struct}<'a>;
impl<'a> $trait for X<'a> $code
								}};",
								    pkt_struct = snake_to_pascal_case(&packet_name) + &version_name.to_uppercase(),
                                );
								if direction == "s2c" {
									inner_post += &format!("const _: () = {{ type X<'a> = ::craftflow_protocol_versions::{direction}::{state_name}::{packet_name}::{version_name}::{pkt_struct}<'a>;
impl<'a> $trait for Post<X<'a>> $code
    								}};",
    								    pkt_struct = snake_to_pascal_case(&packet_name) + &version_name.to_uppercase(),
                                    );
								}
							}
						}
					}
				}
			}
		}
	}

	format!(
		"// This macro is used internally in craftflow for the packet events.
		#[doc(hidden)]
		#[macro_export]
		macro_rules! __gen_impls_for_packets__ {{
		    (impl $trait:ident for X $code:tt) => {{ {inner} }};
			// Instead of making this slop 50x more complicated, we just handle the specific case we need
		    (impl $trait:ident for Post<X> $code:tt) => {{ {inner_post} }};
		}}"
	)
}
