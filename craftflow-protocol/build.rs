//! Generates the packets, their (de)serialization implementations, and conversion between protocol versions
//! from the declarative specification in packets/ directory.

use std::env;
use std::fs;
use std::ops::RangeInclusive;
use std::path::Path;

#[path = "build/parse/mod.rs"]
mod parse;

const SUPPORTED_PROTOCOL_VERSIONS: RangeInclusive<u32> = 700..=800;

fn main() -> anyhow::Result<()> {
	println!("cargo::rerun-if-changed=build.rs");
	println!("cargo::rerun-if-changed=codegen/*");

	let features =
		match parse::feature_definition::parse_features(&fs::read_to_string("codegen/features")?) {
			Ok((_, f)) => f,
			Err(e) => panic!("Failed to parse features: {e:#?}"),
		};

	let states =
		match parse::state_definitions::parse_states(&fs::read_to_string("codegen/states")?) {
			Ok((_, f)) => f,
			Err(e) => panic!("Failed to parse states: {e:#?}"),
		};

	fs::write("test", format!("{:?}", states)).unwrap();

	let out_dir = env::var_os("OUT_DIR").unwrap();
	let dest_path = Path::new(&out_dir).join("generated_packets.rs");

	fs::write(
		&dest_path,
		"pub fn message() -> &'static str {
            \"Hello, World!\"
        }
        ",
	)
	.unwrap();

	Ok(())
}
